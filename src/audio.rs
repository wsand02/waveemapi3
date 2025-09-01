use crate::error::WaveemapiError;
use crate::helpers::mp3_path;
use hound::WavReader;
use mp3lame_encoder::{BuildError, Builder, DualPcm, Encoder, FlushNoGap, MonoPcm};

use std::fs::File;
use std::io::{BufWriter, Write};

use std::cmp;
use std::io::Read;

const CHUNK_SIZE: usize = 1152; // https://stackoverflow.com/questions/72416908/mp3-exact-frame-size-calculation

pub fn wav_decode<R: Read>(
    mut reader: WavReader<R>,
    data_path: &String,
) -> Result<String, WaveemapiError> {
    let channels = reader.spec().channels as usize;
    let bit_depth = reader.spec().bits_per_sample;
    if channels != 1 && channels != 2 {
        return Err(WaveemapiError::HoundError(hound::Error::Unsupported));
    }
    let sample_rate = reader.spec().sample_rate;

    let decode_result = match bit_depth {
        16 => process_samples(
            reader.samples::<i16>(),
            channels,
            1.0 / 32768.0,
            sample_rate,
            data_path,
        )
        .map_err(|e| e)?,
        24 => process_samples(
            reader.samples::<i32>(),
            channels,
            1.0 / 8388608.0,
            sample_rate,
            data_path,
        )
        .map_err(|e| e)?,
        32 => match reader.spec().sample_format {
            hound::SampleFormat::Float => process_samples(
                reader.samples::<f32>(),
                channels,
                1.0,
                sample_rate,
                data_path,
            )
            .map_err(|e| e)?,
            hound::SampleFormat::Int => process_samples(
                reader.samples::<i32>(),
                channels,
                1.0 / 2147483648.0 as f32,
                sample_rate,
                data_path,
            )
            .map_err(|e| e)?,
        },
        _ => return Err(WaveemapiError::HoundError(hound::Error::Unsupported)),
    };
    Ok(decode_result)
}

fn process_samples<T>(
    samples: impl Iterator<Item = hound::Result<T>>,
    channels: usize,
    scale: f32,
    sample_rate: u32,
    data_path: &String,
) -> Result<String, WaveemapiError>
where
    f64: From<T>,
{
    let mut mp3_encoder =
        Builder::new().ok_or_else(|| WaveemapiError::BuildError(BuildError::Generic))?;
    mp3_encoder
        .set_num_channels(channels as u8)
        .expect("set channels");
    mp3_encoder
        .set_sample_rate(sample_rate)
        .map_err(|e| WaveemapiError::BuildError(e))?;
    mp3_encoder
        .set_brate(mp3lame_encoder::Bitrate::Kbps128)
        .map_err(|e| WaveemapiError::BuildError(e))?;
    mp3_encoder
        .set_quality(mp3lame_encoder::Quality::Decent)
        .map_err(|e| WaveemapiError::BuildError(e))?;
    let mut mp3_encoder = mp3_encoder
        .build()
        .map_err(|e| WaveemapiError::BuildError(e))?;
    let ppath = mp3_path(data_path);
    let file = File::create(&ppath).map_err(|e| WaveemapiError::IoError(e))?;
    let mut bwriter = BufWriter::new(file);
    let mut left = Vec::with_capacity(CHUNK_SIZE);
    let mut right = Vec::with_capacity(CHUNK_SIZE);
    let is_stereo = channels == 2;

    for (idx, sample) in samples.enumerate() {
        let sample_val = sample.map_err(|e| WaveemapiError::HoundError(e))?;
        let srb: f64 = sample_val.into();
        let sr: f32 = srb as f32;
        let s = sr * scale;
        if is_stereo {
            if idx % 2 == 0 {
                left.push(s);
            } else {
                right.push(s);
            }
            if left.len() >= CHUNK_SIZE && right.len() >= CHUNK_SIZE {
                encode_dual(
                    &left[..CHUNK_SIZE],
                    &right[..CHUNK_SIZE],
                    &mut bwriter,
                    &mut mp3_encoder,
                )?;
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
    if is_stereo {
        if !left.is_empty() || !right.is_empty() {
            let max_len = std::cmp::max(left.len(), right.len());
            left.resize(max_len, 0.0);
            right.resize(max_len, 0.0);
            encode_dual(&left, &right, &mut bwriter, &mut mp3_encoder)?;
            left.clear();
            right.clear();
        }
    } else {
        if !left.is_empty() {
            encode_mono(&left, &mut bwriter, &mut mp3_encoder)?;
            left.clear();
        }
    }
    let num_frames = cmp::max(left.len(), right.len());
    let mut tail = Vec::with_capacity(mp3lame_encoder::max_required_buffer_size(num_frames));
    let flushed = mp3_encoder.flush_to_vec::<FlushNoGap>(&mut tail)?;
    if flushed > 0 {
        bwriter.write_all(&tail).map_err(WaveemapiError::IoError)?;
    }

    bwriter.flush()?;
    Ok(ppath)
}

fn encode_dual(
    left: &[f32],
    right: &[f32],
    bwriter: &mut BufWriter<File>,
    encoder: &mut Encoder,
) -> Result<(), WaveemapiError> {
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
) -> Result<(), WaveemapiError> {
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
