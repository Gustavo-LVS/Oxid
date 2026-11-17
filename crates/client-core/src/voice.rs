//! Conexão de voz com o SFU (LiveKit).

use livekit::{
    Room, RoomEvent, RoomOptions,
    options::TrackPublishOptions,
    track::{LocalAudioTrack, LocalTrack, TrackSource},
    webrtc::audio_source::{AudioSourceOptions, RtcAudioSource, native::NativeAudioSource},
};
use oxid_proto::voice::VoiceTokenResponse;
use tokio::sync::mpsc::UnboundedReceiver;

pub const SAMPLE_RATE: u32 = 48_000;
pub const CHANNELS: u32 = 1;

/// Sessão de voz conectada, com uma faixa de microfone publicada.
pub struct VoiceSession {
    pub room: Room,
    pub events: UnboundedReceiver<RoomEvent>,
    /// Fonte onde o cliente injeta quadros PCM (10 ms, 48 kHz, mono).
    pub mic: NativeAudioSource,
}

impl VoiceSession {
    pub async fn connect(creds: &VoiceTokenResponse) -> livekit::RoomResult<Self> {
        let (room, events) =
            Room::connect(&creds.url, &creds.token, RoomOptions::default()).await?;
        let mic = NativeAudioSource::new(
            AudioSourceOptions {
                echo_cancellation: true,
                noise_suppression: true,
                auto_gain_control: true,
            },
            SAMPLE_RATE,
            CHANNELS,
            100,
        );
        let track =
            LocalAudioTrack::create_audio_track("microfone", RtcAudioSource::Native(mic.clone()));
        room.local_participant()
            .publish_track(
                LocalTrack::Audio(track),
                TrackPublishOptions {
                    source: TrackSource::Microphone,
                    ..Default::default()
                },
            )
            .await?;
        Ok(Self { room, events, mic })
    }
}
