//! Saída de áudio: mistura as vozes recebidas no alto-falante padrão.

use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

use cpal::{
    SampleFormat, Stream,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use crate::capture::CaptureError;

/// Atraso máximo acumulado por participante; acima disso o áudio mais antigo
/// é descartado (rede instável ou relógios diferentes não viram eco atrasado).
const MAX_BUFFER_MS: usize = 200;

/// Fila de áudio por participante, no formato do dispositivo de saída.
/// Clonável e `Send`: as tarefas de rede empurram, a thread de áudio consome.
#[derive(Clone)]
pub struct Mixer {
    pub sample_rate: u32,
    pub channels: u16,
    queues: Arc<Mutex<HashMap<String, VecDeque<f32>>>>,
}

impl Mixer {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        Self {
            sample_rate,
            channels,
            queues: Default::default(),
        }
    }

    /// Enfileira amostras intercaladas (na taxa e nº de canais do dispositivo)
    /// de um participante. Empurre sempre quadros inteiros.
    pub fn push(&self, who: &str, samples: impl IntoIterator<Item = f32>) {
        let max = self.sample_rate as usize * self.channels as usize * MAX_BUFFER_MS / 1000;
        let mut queues = self.queues.lock().unwrap();
        let q = queues.entry(who.to_string()).or_default();
        q.extend(samples);
        if q.len() > max {
            let extra = q.len() - max;
            q.drain(..extra);
        }
    }

    pub fn remove(&self, who: &str) {
        self.queues.lock().unwrap().remove(who);
    }

    /// Preenche `out` com a soma de todos os participantes (silêncio se faltar áudio).
    pub fn fill(&self, out: &mut [f32]) {
        out.fill(0.0);
        let mut queues = self.queues.lock().unwrap();
        for q in queues.values_mut() {
            let n = q.len().min(out.len());
            for (o, s) in out.iter_mut().zip(q.drain(..n)) {
                *o += s;
            }
        }
        for o in out {
            *o = o.clamp(-1.0, 1.0);
        }
    }
}

/// Reprodução ativa; parar = descartar.
pub struct Playback {
    _stream: Stream,
    pub mixer: Mixer,
}

/// Abre o alto-falante padrão no formato nativo dele. Quem empurra áudio no
/// [`Mixer`] deve usar `mixer.sample_rate` e `mixer.channels`.
pub fn start() -> Result<Playback, CaptureError> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or(CaptureError::NoOutputDevice)?;
    let supported = device.default_output_config()?;
    let format = supported.sample_format();
    let config = supported.config();
    let mixer = Mixer::new(config.sample_rate, config.channels);
    let m = mixer.clone();
    let on_err = |e| eprintln!("erro na reprodução de áudio: {e}");

    let stream = match format {
        SampleFormat::F32 => device.build_output_stream(
            config,
            move |data: &mut [f32], _: &_| m.fill(data),
            on_err,
            None,
        )?,
        SampleFormat::I16 => {
            let mut buf = Vec::new();
            device.build_output_stream(
                config,
                move |data: &mut [i16], _: &_| {
                    buf.resize(data.len(), 0.0);
                    m.fill(&mut buf);
                    for (d, s) in data.iter_mut().zip(&buf) {
                        *d = (s * i16::MAX as f32) as i16;
                    }
                },
                on_err,
                None,
            )?
        }
        other => return Err(CaptureError::UnsupportedFormat(other)),
    };
    stream.play()?;
    Ok(Playback {
        _stream: stream,
        mixer,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixes_participants_and_pads_with_silence() {
        let m = Mixer::new(48_000, 1);
        m.push("ana", [0.25; 4]);
        m.push("bia", [0.5; 2]);
        let mut out = [9.0; 4];
        m.fill(&mut out);
        assert_eq!(out, [0.75, 0.75, 0.25, 0.25]);
        m.fill(&mut out);
        assert_eq!(out, [0.0; 4]);
    }

    #[test]
    fn caps_latency_and_clips() {
        let m = Mixer::new(1_000, 2); // 200 ms = 400 amostras
        m.push("ana", (0..1000).map(|i| i as f32));
        let mut out = [0.0; 2];
        m.fill(&mut out);
        // Sobrou só o fim (as 400 mais recentes) e a soma é limitada a 1.
        assert_eq!(m.queues.lock().unwrap()["ana"].len(), 398);
        assert_eq!(out, [1.0, 1.0]);
    }
}
