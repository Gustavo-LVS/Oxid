//! Supressão de ruído (RNNoise) e decisão de transmitir: ativação por voz ou
//! push-to-talk.

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use nnnoiseless::DenoiseState;

use crate::FRAME_SAMPLES;

/// Quadros iniciais (200 ms) em que a ativação por voz fica fechada enquanto o
/// RNNoise aprende o ruído de fundo; sem isso o próprio início do ruído abre o microfone.
const VAD_WARMUP_FRAMES: u32 = 20;

/// Como o microfone abre.
#[derive(Debug, Clone)]
pub enum TransmitMode {
    /// Transmite enquanto a tecla estiver pressionada.
    PushToTalk(Arc<AtomicBool>),
    /// Transmite quando a probabilidade de voz passa do limiar, e segura por
    /// `hangover_frames` quadros depois que a voz para (evita cortar o fim das palavras).
    VoiceActivity {
        threshold: f32,
        hangover_frames: u32,
    },
}

impl Default for TransmitMode {
    fn default() -> Self {
        Self::VoiceActivity {
            threshold: 0.6,
            hangover_frames: 30,
        }
    }
}

/// Resultado de um quadro processado.
#[derive(Debug, Clone)]
pub struct Processed {
    /// Áudio em [-1, 1]; silêncio quando `transmit` é falso.
    pub samples: [f32; FRAME_SAMPLES],
    /// Probabilidade de voz estimada pelo RNNoise (0 a 1).
    pub voice_probability: f32,
    /// Se o quadro deve ser enviado (também serve para o indicador de fala).
    pub transmit: bool,
}

pub struct AudioProcessor {
    denoise: Box<DenoiseState<'static>>,
    noise_suppression: bool,
    mode: TransmitMode,
    muted: bool,
    hangover_left: u32,
    frames_seen: u32,
    scratch_in: [f32; FRAME_SAMPLES],
    scratch_out: [f32; FRAME_SAMPLES],
}

impl AudioProcessor {
    pub fn new(mode: TransmitMode) -> Self {
        Self {
            denoise: DenoiseState::new(),
            noise_suppression: true,
            mode,
            muted: false,
            hangover_left: 0,
            frames_seen: 0,
            scratch_in: [0.0; FRAME_SAMPLES],
            scratch_out: [0.0; FRAME_SAMPLES],
        }
    }

    pub fn set_mode(&mut self, mode: TransmitMode) {
        self.mode = mode;
        self.hangover_left = 0;
    }

    pub fn set_noise_suppression(&mut self, on: bool) {
        self.noise_suppression = on;
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }

    /// Processa um quadro de 10 ms (mono, 48 kHz, valores em [-1, 1]).
    pub fn process(&mut self, frame: &[f32; FRAME_SAMPLES]) -> Processed {
        // O RNNoise trabalha na escala de i16.
        for (d, s) in self.scratch_in.iter_mut().zip(frame) {
            *d = s * i16::MAX as f32;
        }
        let voice_probability = self
            .denoise
            .process_frame(&mut self.scratch_out, &self.scratch_in);
        self.frames_seen = self.frames_seen.saturating_add(1);
        let mut samples = [0f32; FRAME_SAMPLES];
        if self.noise_suppression {
            for (d, s) in samples.iter_mut().zip(&self.scratch_out) {
                *d = (s / i16::MAX as f32).clamp(-1.0, 1.0);
            }
        } else {
            samples = *frame;
        }
        let transmit = !self.muted && self.decide(voice_probability);
        if !transmit {
            samples = [0.0; FRAME_SAMPLES];
        }
        Processed {
            samples,
            voice_probability,
            transmit,
        }
    }

    fn decide(&mut self, voice_probability: f32) -> bool {
        match &self.mode {
            TransmitMode::PushToTalk(pressed) => pressed.load(Ordering::Relaxed),
            TransmitMode::VoiceActivity {
                threshold,
                hangover_frames,
            } => {
                if self.frames_seen <= VAD_WARMUP_FRAMES {
                    false
                } else if voice_probability >= *threshold {
                    self.hangover_left = *hangover_frames;
                    true
                } else if self.hangover_left > 0 {
                    self.hangover_left -= 1;
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rms;

    /// Ruído de fundo determinístico e "colorido" (passa-baixa), parecido com
    /// ventoinha ou chiado de ambiente; amplitude ~0.1.
    struct Noise {
        seed: u32,
        y: f32,
    }

    impl Noise {
        fn new(seed: u32) -> Self {
            Self { seed, y: 0.0 }
        }

        fn frame(&mut self) -> [f32; FRAME_SAMPLES] {
            let mut f = [0f32; FRAME_SAMPLES];
            for s in &mut f {
                self.seed = self
                    .seed
                    .wrapping_mul(1_664_525)
                    .wrapping_add(1_013_904_223);
                let x = ((self.seed >> 8) as f32 / (1u32 << 24) as f32 - 0.5) * 0.2;
                self.y = 0.95 * self.y + 0.3 * x;
                *s = self.y;
            }
            f
        }
    }

    #[test]
    fn suppresses_steady_noise() {
        let mut p = AudioProcessor::new(TransmitMode::PushToTalk(Arc::new(AtomicBool::new(true))));
        let mut noise = Noise::new(7);
        let (mut before, mut after) = (0.0, 0.0);
        for i in 0..300 {
            let f = noise.frame();
            let out = p.process(&f);
            if i >= 100 {
                before += rms(&f);
                after += rms(&out.samples);
            }
        }
        assert!(
            after < before * 0.1,
            "ruído antes {before:.2}, depois {after:.2}"
        );
    }

    #[test]
    fn push_to_talk_follows_key() {
        let key = Arc::new(AtomicBool::new(false));
        let mut p = AudioProcessor::new(TransmitMode::PushToTalk(key.clone()));
        let mut noise = Noise::new(1);
        let out = p.process(&noise.frame());
        assert!(!out.transmit);
        assert!(out.samples.iter().all(|&s| s == 0.0));
        key.store(true, Ordering::Relaxed);
        assert!(p.process(&noise.frame()).transmit);
        p.set_muted(true);
        assert!(!p.process(&noise.frame()).transmit);
    }

    #[test]
    fn voice_activity_hangover() {
        let mut p = AudioProcessor::new(TransmitMode::VoiceActivity {
            threshold: 0.5,
            hangover_frames: 3,
        });
        p.frames_seen = VAD_WARMUP_FRAMES + 1;
        let seq = [0.1, 0.9, 0.2, 0.2, 0.2, 0.2, 0.7, 0.1];
        let got: Vec<bool> = seq.iter().map(|&v| p.decide(v)).collect();
        assert_eq!(got, [false, true, true, true, true, false, true, true]);
    }

    #[test]
    fn noise_alone_does_not_open_voice_activity() {
        let mut p = AudioProcessor::new(TransmitMode::default());
        let mut noise = Noise::new(3);
        let opened = (0..300)
            .filter(|_| p.process(&noise.frame()).transmit)
            .count();
        assert_eq!(opened, 0, "abriu em {opened} de 300 quadros só com ruído");
    }
}
