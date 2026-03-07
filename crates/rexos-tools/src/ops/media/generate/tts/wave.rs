pub(super) fn placeholder_tts_wave() -> Vec<u8> {
    let sample_rate: u32 = 16_000;
    let duration_ms: u32 = 300;
    let num_samples = (sample_rate as usize)
        .saturating_mul(duration_ms as usize)
        .saturating_div(1000);
    let frequency_hz: f32 = 440.0;
    let amplitude: f32 = 0.20;
    let data_size = num_samples * 2;
    let chunk_size = 36 + data_size as u32;
    let byte_rate = sample_rate * 2;

    let mut bytes = Vec::with_capacity(44 + data_size);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&chunk_size.to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    bytes.extend_from_slice(&byte_rate.to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(&16u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&(data_size as u32).to_le_bytes());

    for sample_index in 0..num_samples {
        let time_secs = sample_index as f32 / sample_rate as f32;
        let sample = tone_sample(time_secs, frequency_hz, amplitude);
        bytes.extend_from_slice(&sample.to_le_bytes());
    }

    bytes
}

fn tone_sample(time_secs: f32, frequency_hz: f32, amplitude: f32) -> i16 {
    let wave = (2.0 * std::f32::consts::PI * frequency_hz * time_secs).sin();
    (wave * amplitude * i16::MAX as f32) as i16
}
