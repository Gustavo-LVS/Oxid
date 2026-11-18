//! Áudio do cliente Oxid.
//!
//! Fluxo: microfone (qualquer taxa e nº de canais) -> [`frames::FrameAssembler`]
//! (mono, 48 kHz, quadros de 10 ms) -> [`processor::AudioProcessor`] (supressão
//! de ruído RNNoise + ativação por voz ou push-to-talk) -> SFU.
//!
//! Cancelamento de eco e controle automático de ganho ficam no processamento
//! WebRTC do LiveKit (`AudioSourceOptions` em `client-core::voice`).

#[cfg(feature = "capture")]
pub mod capture;
pub mod frames;
#[cfg(feature = "capture")]
pub mod playback;
pub mod processor;

/// Taxa de amostragem interna (a mesma do Opus e do RNNoise).
pub const SAMPLE_RATE: u32 = 48_000;
/// Amostras por quadro: 10 ms a 48 kHz (tamanho de quadro do RNNoise).
pub const FRAME_SAMPLES: usize = 480;

/// Volume RMS de um quadro em [-1, 1].
pub fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt()
}
