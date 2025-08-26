use crate::error::WavError;
use hound::WavReader;
use std::io::Read;

pub fn wav_decode<R: Read>(mut reader: WavReader<R>) -> Result<(Vec<f32>, Vec<f32>), WavError> {
    let channels = reader.spec().channels as usize;
    let bit_depth = reader.spec().bits_per_sample;

    let decode_result = match bit_depth {
        16 => process_samples(reader.samples::<i16>(), channels, 1.0 / 32768.0).map_err(|e| e)?,
        24 => process_samples(reader.samples::<i32>(), channels, 1.0 / 8388608.0).map_err(|e| e)?,
        32 => match reader.spec().sample_format {
            hound::SampleFormat::Float => {
                process_samples(reader.samples::<f32>(), channels, 1.0).map_err(|e| e)?
            }
            hound::SampleFormat::Int => {
                process_samples(reader.samples::<i32>(), channels, 1.0 / 2147483648.0 as f32)
                    .map_err(|e| e)?
            }
        },
        _ => return Err(WavError::HoundError(hound::Error::Unsupported)),
    };
    Ok(decode_result)
}

pub fn process_samples<T>(
    samples: impl Iterator<Item = hound::Result<T>>,
    channels: usize,
    scale: f32,
) -> Result<(Vec<f32>, Vec<f32>), WavError>
where
    f64: From<T>,
{
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut idx = 0;
    for sample in samples {
        let sample_val = sample.map_err(|e| WavError::HoundError(e))?;
        let srb: f64 = sample_val.into();
        let sr: f32 = srb as f32;
        let s = sr * scale;
        match channels {
            1 => left.push(s),
            2 => {
                if idx % 2 == 0 {
                    left.push(s);
                } else {
                    right.push(s);
                }
                idx += 1;
            }
            _ => {}
        }
    }
    Ok((left, right))
}
