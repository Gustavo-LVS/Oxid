//! Pipeline completo com fala real (sintetizada) + ruído de fundo:
//! 1 s de ruído -> fala com ruído -> 1,5 s de ruído.

use oxid_client_audio::{
    FRAME_SAMPLES,
    frames::FrameAssembler,
    processor::{AudioProcessor, TransmitMode},
    rms,
};

fn speech() -> (Vec<f32>, u32) {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fala-ptbr.wav");
    let mut r = hound::WavReader::open(path).unwrap();
    let rate = r.spec().sample_rate;
    let s = r
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();
    (s, rate)
}

/// Ruído colorido de fundo, amplitude ~0.03.
fn noise(n: usize, seed: &mut u32, y: &mut f32) -> Vec<f32> {
    (0..n)
        .map(|_| {
            *seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let x = ((*seed >> 8) as f32 / (1u32 << 24) as f32 - 0.5) * 0.06;
            *y = 0.95 * *y + 0.3 * x;
            *y
        })
        .collect()
}

#[test]
fn voice_activity_opens_on_speech_and_closes_on_noise() {
    let (fala, rate) = speech();
    let (mut seed, mut y) = (11u32, 0f32);
    let lead = rate as usize;
    let tail = rate as usize * 3 / 2;
    let mut signal = noise(lead, &mut seed, &mut y);
    let bg = noise(fala.len(), &mut seed, &mut y);
    signal.extend(fala.iter().zip(&bg).map(|(s, n)| s + n));
    signal.extend(noise(tail, &mut seed, &mut y));

    let mut asm = FrameAssembler::new(rate, 1);
    let mut frames: Vec<[f32; FRAME_SAMPLES]> = Vec::new();
    for chunk in signal.chunks(512) {
        asm.push(chunk, |f| frames.push(*f));
    }
    let mut p = AudioProcessor::new(TransmitMode::default());
    let out: Vec<_> = frames.iter().map(|f| p.process(f)).collect();

    // Regiões em quadros de 10 ms.
    let to_frames = |samples: usize| samples * 100 / rate as usize;
    let speech_start = to_frames(lead);
    let speech_end = to_frames(lead + fala.len());
    let lead_open = out[..speech_start].iter().filter(|o| o.transmit).count();
    let speech_open = out[speech_start..speech_end]
        .iter()
        .filter(|o| o.transmit)
        .count();
    // Depois do hangover (300 ms) + 200 ms de folga, tem que fechar.
    let tail_open = out[speech_end + 50..].iter().filter(|o| o.transmit).count();
    let speech_frames = speech_end - speech_start;
    println!("antes {lead_open}, fala {speech_open}/{speech_frames}, depois {tail_open}");

    assert_eq!(lead_open, 0, "abriu com ruído antes da fala");
    assert!(
        speech_open * 10 >= speech_frames * 7,
        "abriu só em {speech_open}/{speech_frames} quadros de fala"
    );
    assert_eq!(tail_open, 0, "não fechou depois da fala");

    // Supressão: a fala sobrevive e o ruído some.
    let energy = |r: std::ops::Range<usize>, processed: bool| -> f32 {
        r.map(|i| {
            if processed {
                rms(&out[i].samples)
            } else {
                rms(&frames[i])
            }
        })
        .sum()
    };
    let p2 = {
        let mut q = AudioProcessor::new(TransmitMode::PushToTalk(std::sync::Arc::new(true.into())));
        frames
            .iter()
            .map(|f| q.process(f).samples)
            .collect::<Vec<_>>()
    };
    let noise_in: f32 = (20..speech_start).map(|i| rms(&frames[i])).sum();
    let noise_out: f32 = (20..speech_start).map(|i| rms(&p2[i])).sum();
    let speech_in = energy(speech_start..speech_end, false);
    let speech_out: f32 = (speech_start..speech_end).map(|i| rms(&p2[i])).sum();
    println!("ruído {noise_in:.2} -> {noise_out:.2}; fala {speech_in:.2} -> {speech_out:.2}");
    assert!(noise_out < noise_in * 0.1, "ruído não foi suprimido");
    assert!(speech_out > speech_in * 0.5, "a fala foi cortada demais");
}
