//! Prova de conceito do passo 2: dois clientes falando via SFU.
//!
//! Cada instância faz login na API, pede um token de voz, entra na sala,
//! publica um tom senoidal (no lugar do microfone, que chega no passo 5) e
//! imprime o volume de cada participante remoto que ela está ouvindo.
//!
//! Uso (em dois terminais, com usuários diferentes):
//!   OXID_API=http://localhost:8080 OXID_LOGIN=ana OXID_PASSWORD=... OXID_ROOM=sala-1 OXID_TONE_HZ=440 \
//!     cargo run -p oxid-client-core --example voice_poc

use std::{borrow::Cow, collections::HashMap, env, f32::consts::TAU, time::Duration};

use futures_util::StreamExt;
use livekit::{
    RoomEvent,
    track::RemoteTrack,
    webrtc::{audio_frame::AudioFrame, audio_stream::native::NativeAudioStream},
};
use oxid_client_core::{
    api::ApiClient,
    voice::{CHANNELS, SAMPLE_RATE, VoiceSession},
};

fn var(name: &str, default: Option<&str>) -> anyhow::Result<String> {
    env::var(name).or_else(|_| {
        default
            .map(String::from)
            .ok_or_else(|| anyhow::anyhow!("defina {name}"))
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_url = var("OXID_API", Some("http://localhost:8080"))?;
    let login = var("OXID_LOGIN", None)?;
    let password = var("OXID_PASSWORD", None)?;
    let room = var("OXID_ROOM", Some("sala-1"))?;
    let tone_hz: f32 = var("OXID_TONE_HZ", Some("440"))?.parse()?;

    let mut api = ApiClient::new(api_url);
    let me = api.login(&login, &password).await?;
    let creds = api.voice_token(&room).await?;
    println!(
        "{} entrando na sala {room} em {}",
        me.user.username, creds.url
    );

    let mut session = VoiceSession::connect(&creds).await?;
    println!("conectado; publicando tom de {tone_hz} Hz");

    // Gera 10 ms de áudio por vez, como um microfone faria.
    let mic = session.mic.clone();
    tokio::spawn(async move {
        let spc = (SAMPLE_RATE / 100) as usize;
        let mut phase = 0f32;
        let step = TAU * tone_hz / SAMPLE_RATE as f32;
        let mut tick = tokio::time::interval(Duration::from_millis(10));
        loop {
            tick.tick().await;
            let data: Vec<i16> = (0..spc)
                .map(|_| {
                    phase = (phase + step) % TAU;
                    (phase.sin() * 0.3 * i16::MAX as f32) as i16
                })
                .collect();
            let frame = AudioFrame {
                data: Cow::Owned(data),
                sample_rate: SAMPLE_RATE,
                num_channels: CHANNELS,
                samples_per_channel: spc as u32,
            };
            if mic.capture_frame(&frame).await.is_err() {
                break;
            }
        }
    });

    let levels = std::sync::Arc::new(std::sync::Mutex::new(HashMap::<String, f32>::new()));
    let printer = levels.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            for (who, rms) in printer.lock().unwrap().iter() {
                println!("ouvindo {who}: volume RMS {rms:.3}");
            }
        }
    });

    while let Some(event) = session.events.recv().await {
        match event {
            RoomEvent::ParticipantConnected(p) => println!("entrou: {}", p.name()),
            RoomEvent::ParticipantDisconnected(p) => {
                println!("saiu: {}", p.name());
                levels.lock().unwrap().remove(&p.name());
            }
            RoomEvent::TrackSubscribed {
                track: RemoteTrack::Audio(track),
                participant,
                ..
            } => {
                let who = participant.name();
                println!("recebendo áudio de {who}");
                let levels = levels.clone();
                tokio::spawn(async move {
                    let mut stream = NativeAudioStream::new(
                        track.rtc_track(),
                        SAMPLE_RATE as i32,
                        CHANNELS as i32,
                    );
                    while let Some(frame) = stream.next().await {
                        let sum: f64 = frame
                            .data
                            .iter()
                            .map(|&s| (s as f64 / i16::MAX as f64).powi(2))
                            .sum();
                        let rms = (sum / frame.data.len().max(1) as f64).sqrt() as f32;
                        levels.lock().unwrap().insert(who.clone(), rms);
                    }
                });
            }
            RoomEvent::Disconnected { reason } => {
                println!("desconectado: {reason:?}");
                break;
            }
            _ => {}
        }
    }
    Ok(())
}
