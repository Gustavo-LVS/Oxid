//! Converte o áudio bruto do dispositivo em quadros mono de 10 ms a 48 kHz.

use crate::{FRAME_SAMPLES, SAMPLE_RATE};

/// Recebe blocos de tamanho arbitrário (intercalados, qualquer taxa) e
/// devolve quadros completos de [`FRAME_SAMPLES`] amostras.
pub struct FrameAssembler {
    channels: usize,
    /// Passo de leitura na entrada por amostra de saída.
    step: f64,
    /// Posição fracionária da próxima amostra de saída, relativa a `prev`.
    pos: f64,
    /// Última amostra mono de entrada (para interpolar entre blocos).
    prev: f32,
    pending: Vec<f32>,
}

impl FrameAssembler {
    pub fn new(input_rate: u32, channels: u16) -> Self {
        assert!(input_rate > 0 && channels > 0);
        Self {
            channels: channels as usize,
            step: input_rate as f64 / SAMPLE_RATE as f64,
            pos: 0.0,
            prev: 0.0,
            pending: Vec::with_capacity(FRAME_SAMPLES * 2),
        }
    }

    /// Adiciona amostras intercaladas e chama `on_frame` para cada quadro pronto.
    pub fn push(&mut self, interleaved: &[f32], mut on_frame: impl FnMut(&[f32; FRAME_SAMPLES])) {
        for chunk in interleaved.chunks_exact(self.channels) {
            // Mixa os canais em mono.
            let cur = chunk.iter().sum::<f32>() / self.channels as f32;
            // Interpolação linear entre `prev` (pos 0) e `cur` (pos 1).
            while self.pos <= 1.0 {
                let t = self.pos as f32;
                self.pending.push(self.prev + (cur - self.prev) * t);
                self.pos += self.step;
            }
            self.pos -= 1.0;
            self.prev = cur;
        }
        let full = self.pending.len() / FRAME_SAMPLES * FRAME_SAMPLES;
        for frame in self.pending[..full].as_chunks::<FRAME_SAMPLES>().0 {
            on_frame(frame);
        }
        self.pending.drain(..full);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect(asm: &mut FrameAssembler, input: &[f32]) -> Vec<f32> {
        let mut out = Vec::new();
        asm.push(input, |f| out.extend_from_slice(f));
        out
    }

    #[test]
    fn passthrough_48k_mono_in_odd_chunks() {
        let mut asm = FrameAssembler::new(48_000, 1);
        let input: Vec<f32> = (0..1500).map(|i| (i as f32 / 1500.0) - 0.5).collect();
        let mut out = Vec::new();
        for chunk in input.chunks(137) {
            out.extend(collect(&mut asm, chunk));
        }
        assert_eq!(out.len(), 1440); // 3 quadros completos
        // Atraso de 1 amostra pela interpolação; valores preservados.
        for (o, i) in out[1..].iter().zip(&input) {
            assert!((o - i).abs() < 1e-5);
        }
    }

    #[test]
    fn stereo_is_downmixed() {
        let mut asm = FrameAssembler::new(48_000, 2);
        let input: Vec<f32> = (0..2000).flat_map(|_| [0.8, 0.2]).collect();
        let out = collect(&mut asm, &input);
        assert!(out[10..].iter().all(|s| (s - 0.5).abs() < 1e-6));
    }

    #[test]
    fn resamples_44k1_to_48k() {
        let mut asm = FrameAssembler::new(44_100, 1);
        // 1 s de seno de 1 kHz a 44.1 kHz.
        let input: Vec<f32> = (0..44_100)
            .map(|i| (i as f32 * 2.0 * std::f32::consts::PI * 1000.0 / 44_100.0).sin())
            .collect();
        let out = collect(&mut asm, &input);
        assert!(
            (47_520..=48_000).contains(&out.len()),
            "len = {}",
            out.len()
        );
        // Conta cruzamentos por zero: ~2000 para 1 kHz em 1 s.
        let crossings = out.windows(2).filter(|w| w[0] < 0.0 && w[1] >= 0.0).count();
        assert!(
            (990..=1010).contains(&crossings),
            "cruzamentos = {crossings}"
        );
    }
}
