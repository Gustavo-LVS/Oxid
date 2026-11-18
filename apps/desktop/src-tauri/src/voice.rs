//! Voz dentro do app: microfone -> RNNoise + ativação por voz/push-to-talk ->
//! LiveKit, e o áudio dos outros participantes -> alto-falante.

use std::{
    borrow::Cow,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use futures_util::StreamExt;
use oxid_client_core::voice::{
    CHANNELS, SAMPLE_RATE, VoiceSession,
    livekit::{
        Room, RoomEvent,
        track::RemoteTrack,
        webrtc::{audio_frame::AudioFrame, audio_stream::native::NativeAudioStream},
    },
    oxid_client_audio::{
        FRAME_SAMPLES, capture,
        playback::{self, Mixer},
        processor::{AudioProcessor, TransmitMode},
    },
};
use oxid_proto::voice::VoiceTokenResponse;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::{sync::Mutex, task::JoinHandle};
use uuid::Uuid;

/// Estado da voz compartilhado com os comandos do Tauri.
#[derive(Default)]
pub struct Voice {
    active: Mutex<Option<Active>>,
    pub muted: Arc<AtomicBool>,
    pub use_ptt: Arc<AtomicBool>,
}

struct Active {
    room: Arc<Room>,
    tasks: Vec<JoinHandle<()>>,
    /// Ao ser descartado, a thread de áudio fecha microfone e alto-falante.
    _audio: std::sync::mpsc::Sender<()>,
}

#[derive(Clone, Serialize)]
struct Member {
    id: String,
    name: String,
}

/// Enviado à interface sempre que alguém entra, sai ou começa/para de falar.
#[derive(Clone, Serialize)]
struct VoiceState {
    channel_id: Uuid,
    members: Vec<Member>,
    speaking: Vec<String>,
}

pub const VOICE_EVENT: &str = "voice";
pub const VOICE_CLOSED: &str = "voice-closed";

/// cpal guarda os streams numa thread própria (eles não são `Send` em todo
/// sistema). Devolve o mixer do alto-falante e a "chave" que encerra a thread.
fn start_audio(
    mic_tx: tokio::sync::mpsc::Sender<[f32; FRAME_SAMPLES]>,
) -> Result<(Mixer, std::sync::mpsc::Sender<()>), String> {
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let (stop_tx, stop_rx) = std::sync::mpsc::channel::<()>();
    std::thread::spawn(move || {
        let streams = capture::start(move |f| {
            let _ = mic_tx.try_send(*f);
        })
        .and_then(|c| playback::start().map(|p| (c, p)));
        match streams {
            Ok((cap, play)) => {
                let _ = ready_tx.send(Ok(play.mixer.clone()));
                let _ = stop_rx.recv(); // bloqueia até o Sender ser descartado
                drop((cap, play));
            }
            Err(e) => {
                let _ = ready_tx.send(Err(e.to_string()));
            }
        }
    });
    let mixer = ready_rx
        .recv()
        .map_err(|_| "a thread de áudio parou".to_string())??;
    Ok((mixer, stop_tx))
}

fn snapshot(room: &Room, channel_id: Uuid, speaking: &[String]) -> VoiceState {
    let local = room.local_participant();
    let mut members = vec![Member {
        id: local.identity().to_string(),
        name: local.name(),
    }];
    members.extend(room.remote_participants().values().map(|p| Member {
        id: p.identity().to_string(),
        name: p.name(),
    }));
    VoiceState {
        channel_id,
        members,
        speaking: speaking.to_vec(),
    }
}

impl Voice {
    pub async fn join(
        &self,
        app: AppHandle,
        creds: VoiceTokenResponse,
        channel_id: Uuid,
        ptt_pressed: Arc<AtomicBool>,
    ) -> Result<(), String> {
        self.leave().await;

        let (mic_tx, mut mic_rx) = tokio::sync::mpsc::channel(64);
        let (mixer, audio) = start_audio(mic_tx)?;
        let session = VoiceSession::connect(&creds)
            .await
            .map_err(|e| format!("não foi possível conectar na voz: {e}"))?;
        let room = Arc::new(session.room);
        let mut events = session.events;

        // Microfone -> processamento -> SFU.
        let mic = session.mic;
        let (muted, use_ptt) = (self.muted.clone(), self.use_ptt.clone());
        let mic_task = tokio::spawn(async move {
            let mut processor = AudioProcessor::new(TransmitMode::default());
            let mut ptt_mode = false;
            while let Some(f) = mic_rx.recv().await {
                let want_ptt = use_ptt.load(Ordering::Relaxed);
                if want_ptt != ptt_mode {
                    ptt_mode = want_ptt;
                    processor.set_mode(if ptt_mode {
                        TransmitMode::PushToTalk(ptt_pressed.clone())
                    } else {
                        TransmitMode::default()
                    });
                }
                processor.set_muted(muted.load(Ordering::Relaxed));
                let out = processor.process(&f);
                let frame = AudioFrame {
                    data: Cow::Owned(
                        out.samples
                            .iter()
                            .map(|s| (s * i16::MAX as f32) as i16)
                            .collect(),
                    ),
                    sample_rate: SAMPLE_RATE,
                    num_channels: CHANNELS,
                    samples_per_channel: FRAME_SAMPLES as u32,
                };
                if mic.capture_frame(&frame).await.is_err() {
                    break;
                }
            }
        });

        // Eventos da sala: quem está, quem fala e o áudio de cada um.
        let r = room.clone();
        let event_task = tokio::spawn(async move {
            let mut speaking: Vec<String> = Vec::new();
            let _ = app.emit(VOICE_EVENT, snapshot(&r, channel_id, &speaking));
            while let Some(ev) = events.recv().await {
                match ev {
                    RoomEvent::TrackSubscribed {
                        track: RemoteTrack::Audio(track),
                        participant,
                        ..
                    } => {
                        let who = participant.identity().to_string();
                        let mixer = mixer.clone();
                        tokio::spawn(async move {
                            let mut stream = NativeAudioStream::new(
                                track.rtc_track(),
                                mixer.sample_rate as i32,
                                mixer.channels as i32,
                            );
                            while let Some(frame) = stream.next().await {
                                mixer.push(
                                    &who,
                                    frame.data.iter().map(|&s| s as f32 / i16::MAX as f32),
                                );
                            }
                            mixer.remove(&who);
                        });
                    }
                    RoomEvent::ActiveSpeakersChanged { speakers } => {
                        speaking = speakers.iter().map(|p| p.identity().to_string()).collect();
                    }
                    RoomEvent::ParticipantConnected(_) | RoomEvent::ParticipantDisconnected(_) => {}
                    RoomEvent::Disconnected { .. } => break,
                    _ => continue,
                }
                let _ = app.emit(VOICE_EVENT, snapshot(&r, channel_id, &speaking));
            }
            let _ = app.emit(VOICE_CLOSED, ());
        });

        *self.active.lock().await = Some(Active {
            room,
            tasks: vec![mic_task, event_task],
            _audio: audio,
        });
        Ok(())
    }

    pub async fn leave(&self) {
        if let Some(active) = self.active.lock().await.take() {
            for t in &active.tasks {
                t.abort();
            }
            let _ = active.room.close().await;
        }
    }
}
