//! Captura do microfone padrão com cpal, entregando quadros de 10 ms.

use cpal::{
    SampleFormat, Stream,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use crate::{FRAME_SAMPLES, frames::FrameAssembler};

#[derive(Debug, thiserror::Error)]
pub enum CaptureError {
    #[error("nenhum microfone encontrado")]
    NoDevice,
    #[error("formato de amostra não suportado: {0:?}")]
    UnsupportedFormat(SampleFormat),
    #[error("erro do dispositivo de áudio: {0}")]
    Device(#[from] cpal::Error),
}

/// Captura ativa; parar = descartar.
pub struct Capture {
    _stream: Stream,
    pub device_name: String,
}

/// Abre o microfone padrão e chama `on_frame` (na thread de áudio) para cada
/// quadro mono de 10 ms a 48 kHz. `on_frame` deve ser rápido: só repasse o quadro.
pub fn start(
    mut on_frame: impl FnMut(&[f32; FRAME_SAMPLES]) + Send + 'static,
) -> Result<Capture, CaptureError> {
    let host = cpal::default_host();
    let device = host.default_input_device().ok_or(CaptureError::NoDevice)?;
    let device_name = device
        .description()
        .map(|d| d.to_string())
        .unwrap_or_else(|_| "microfone".into());
    let supported = device.default_input_config()?;
    let format = supported.sample_format();
    let config = supported.config();
    let mut asm = FrameAssembler::new(config.sample_rate, config.channels);
    let on_err = |e| eprintln!("erro na captura de áudio: {e}");

    let stream = match format {
        SampleFormat::F32 => device.build_input_stream(
            config,
            move |data: &[f32], _: &_| asm.push(data, &mut on_frame),
            on_err,
            None,
        )?,
        SampleFormat::I16 => {
            let mut buf = Vec::new();
            device.build_input_stream(
                config,
                move |data: &[i16], _: &_| {
                    buf.clear();
                    buf.extend(data.iter().map(|&s| s as f32 / i16::MAX as f32));
                    asm.push(&buf, &mut on_frame);
                },
                on_err,
                None,
            )?
        }
        other => return Err(CaptureError::UnsupportedFormat(other)),
    };
    stream.play()?;
    Ok(Capture {
        _stream: stream,
        device_name,
    })
}
