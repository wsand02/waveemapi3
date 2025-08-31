use crate::error::ObaError;
use hound::WavReader;
use mp3lame_encoder::{BuildError, Builder, DualPcm, Encoder, MonoPcm};
use std::fs::File;
use std::io::{BufWriter, Write};

use std::cmp;
use std::{io::Read, path::Path};
use uuid::Uuid;

pub const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "uploads");
const CHUNK_SIZE: usize = 1152; // https://stackoverflow.com/questions/72416908/mp3-exact-frame-size-calculation

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

fn process_samples<T>(
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
    let mut left = Vec::with_capacity(CHUNK_SIZE);
    let mut right = Vec::with_capacity(CHUNK_SIZE);
    let is_stereo = channels == 2;

    for (idx, sample) in samples.enumerate() {
        let sample_val = sample.map_err(|e| ObaError::HoundError(e))?;
        let srb: f64 = sample_val.into();
        let sr: f32 = srb as f32;
        let s = sr * scale;
        if is_stereo {
            if idx % 2 == 0 {
                left.push(s);
            } else {
                right.push(s);
            }
            if left.len() >= CHUNK_SIZE {
                encode_dual(&left, &right, &mut bwriter, &mut mp3_encoder)?;
                left.clear();
                right.clear();
            }
        } else {
            left.push(s);
            if left.len() >= CHUNK_SIZE {
                encode_mono(&left, &mut bwriter, &mut mp3_encoder)?;
                left.clear();
            }
        }
    }
    if !left.is_empty() && !right.is_empty() {
        if !right.is_empty() {
            encode_dual(&left, &right, &mut bwriter, &mut mp3_encoder)?;
        } else {
            encode_mono(&left, &mut bwriter, &mut mp3_encoder)?;
        }
    }

    Ok(ppath.to_string_lossy().to_string())
}

fn encode_dual(
    left: &[f32],
    right: &[f32],
    bwriter: &mut BufWriter<File>,
    encoder: &mut Encoder,
) -> Result<(), ObaError> {
    let chunk = DualPcm {
        left: left,
        right: right,
    };
    let num_frames = cmp::max(left.len(), right.len());
    let mut mp3_out_buffer =
        Vec::with_capacity(mp3lame_encoder::max_required_buffer_size(num_frames));
    let encoded = encoder.encode_to_vec(chunk, &mut mp3_out_buffer)?;
    if encoded > 0 {
        bwriter.write_all(&mp3_out_buffer)?;
    }
    Ok(())
}

fn encode_mono(
    left: &[f32],
    bwriter: &mut BufWriter<File>,
    encoder: &mut Encoder,
) -> Result<(), ObaError> {
    let chunk = MonoPcm(left);
    let num_frames = left.len();
    let mut mp3_out_buffer =
        Vec::with_capacity(mp3lame_encoder::max_required_buffer_size(num_frames));
    let encoded = encoder.encode_to_vec(chunk, &mut mp3_out_buffer)?;
    if encoded > 0 {
        bwriter.write_all(&mp3_out_buffer)?;
    }
    Ok(())
}
