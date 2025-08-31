use crate::error::ObaError;
use core::panic;
use hound::WavReader;
use mp3lame_encoder::{BuildError, Builder, DualPcm};
use std::fs::File;
use std::io::{BufWriter, Write};

use std::cmp;
use std::{io::Read, path::Path};
use uuid::Uuid;

pub const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "uploads");

pub fn wav_decode<R: Read>(mut reader: WavReader<R>) -> Result<String, ObaError> {
    let channels = reader.spec().channels as usize;
    let bit_depth = reader.spec().bits_per_sample;
    if channels != 1 && channels != 2 {
        return Err(ObaError::HoundError(hound::Error::Unsupported));
    }
    let sample_rate = reader.spec().sample_rate;

    let decode_result = match bit_depth {
        16 => process_samples(
            reader.samples::<i16>(),
            channels,
            1.0 / 32768.0,
            sample_rate,
        )
        .map_err(|e| e)?,
        24 => process_samples(
            reader.samples::<i32>(),
            channels,
            1.0 / 8388608.0,
            sample_rate,
        )
        .map_err(|e| e)?,
        32 => match reader.spec().sample_format {
            hound::SampleFormat::Float => {
                process_samples(reader.samples::<f32>(), channels, 1.0, sample_rate)
                    .map_err(|e| e)?
            }
            hound::SampleFormat::Int => process_samples(
                reader.samples::<i32>(),
                channels,
                1.0 / 2147483648.0 as f32,
                sample_rate,
            )
            .map_err(|e| e)?,
        },
        _ => return Err(ObaError::HoundError(hound::Error::Unsupported)),
    };
    Ok(decode_result)
}

pub fn process_samples<T>(
    samples: impl Iterator<Item = hound::Result<T>>,
    channels: usize,
    scale: f32,
    sample_rate: u32,
) -> Result<String, ObaError>
where
    f64: From<T>,
{
    let mut mp3_encoder =
        Builder::new().ok_or_else(|| ObaError::BuildError(BuildError::Generic))?;
    mp3_encoder
        .set_num_channels(channels as u8)
        .expect("set channels");
    mp3_encoder
        .set_sample_rate(sample_rate)
        .map_err(|e| ObaError::BuildError(e))?;
    mp3_encoder
        .set_brate(mp3lame_encoder::Bitrate::Kbps128)
        .map_err(|e| ObaError::BuildError(e))?;
    mp3_encoder
        .set_quality(mp3lame_encoder::Quality::Decent)
        .map_err(|e| ObaError::BuildError(e))?;
    let mut mp3_encoder = mp3_encoder.build().map_err(|e| ObaError::BuildError(e))?;
    let ppath = Path::new(ROOT).join(Uuid::new_v4().to_string());
    let file = File::create(&ppath).map_err(|e| ObaError::IoError(e))?;
    let mut bwriter = BufWriter::new(file);
    let chunk_size = 1024;
    let mut left = vec![0_f32; chunk_size];
    let mut right = vec![0_f32; chunk_size];
    let mut idx = 0;

    for sample in samples {
        let sample_val = sample.map_err(|e| ObaError::HoundError(e))?;
        let srb: f64 = sample_val.into();
        let sr: f32 = srb as f32;
        let s = sr * scale;
        let chunk_ready = match channels {
            1 => left.len() >= chunk_size,
            2 => left.len() >= chunk_size && right.len() >= chunk_size,
            _ => panic!("Yeah... you shouldn't be here"),
        };
        if chunk_ready {
            let chunk = DualPcm {
                left: &left,
                right: &right,
            };
            let num_frames = cmp::max(left.len(), right.len());
            let mut mp3_out_buffer =
                Vec::with_capacity(mp3lame_encoder::max_required_buffer_size(num_frames));
            let _encoded = mp3_encoder.encode_to_vec(chunk, &mut mp3_out_buffer);
            let _ = bwriter.write_all(&mp3_out_buffer);
            left.clear();
            right.clear();
        }
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
            _ => panic!("You shouldn't be here"),
        }
    }
    Ok(ppath.to_string_lossy().to_string())
}
