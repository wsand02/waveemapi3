use crate::error::Mp3Error;
use mp3lame_encoder::{BuildError, Builder, DualPcm, Encoder, FlushNoGap, MonoPcm};
use std::cmp;

pub fn mp3_encode(left: &Vec<f32>, right: &Vec<f32>) -> Result<Vec<u8>, Mp3Error> {
    let channels = get_num_channels(left, right);

    let chunk_size = 1024;
    let mut mp3_encoder =
        Builder::new().ok_or_else(|| Mp3Error::BuildError(BuildError::Generic))?;
    mp3_encoder
        .set_num_channels(channels)
        .expect("set channels");
    mp3_encoder
        .set_sample_rate(44_100)
        .map_err(|e| Mp3Error::BuildError(e))?;
    mp3_encoder
        .set_brate(mp3lame_encoder::Bitrate::Kbps128)
        .map_err(|e| Mp3Error::BuildError(e))?;
    mp3_encoder
        .set_quality(mp3lame_encoder::Quality::Decent)
        .map_err(|e| Mp3Error::BuildError(e))?;

    let mut mp3_encoder = mp3_encoder.build().map_err(|e| Mp3Error::BuildError(e))?;
    let num_frames = cmp::max(left.len(), right.len());
    let mut mp3_out_buffer =
        Vec::with_capacity(mp3lame_encoder::max_required_buffer_size(num_frames));
    if channels == 1 {
        chunked_encode_mono(&mut mp3_encoder, left, chunk_size, &mut mp3_out_buffer)?;
    } else {
        chunked_encode_dual(
            &mut mp3_encoder,
            left,
            right,
            chunk_size,
            &mut mp3_out_buffer,
        )?;
    }

    let written = mp3_encoder
        .flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut())
        .map_err(Mp3Error::EncoderError)?;
    unsafe {
        mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(written));
    }
    Ok(mp3_out_buffer)
}

fn chunked_encode_dual(
    mp3_encoder: &mut Encoder,
    left: &[f32],
    right: &[f32],
    chunk_size: usize,
    mp3_out_buffer: &mut Vec<u8>,
) -> Result<(), Mp3Error> {
    let mut i = 0;
    while i < left.len() {
        let end = (i + chunk_size).min(left.len());
        let chunk = DualPcm {
            left: &left[i..end],
            right: &right[i..end],
        };
        let encoded = mp3_encoder
            .encode(chunk, mp3_out_buffer.spare_capacity_mut())
            .map_err(|e| Mp3Error::EncoderError(e))?;
        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded));
        }
        i += chunk_size;
    }
    Ok(())
}

fn chunked_encode_mono(
    mp3_encoder: &mut Encoder,
    left: &[f32],
    chunk_size: usize,
    mp3_out_buffer: &mut Vec<u8>,
) -> Result<(), Mp3Error> {
    let mut i = 0;
    while i < left.len() {
        let end = (i + chunk_size).min(left.len());
        let chunk = MonoPcm(&left[i..end]);
        let encoded = mp3_encoder
            .encode(chunk, mp3_out_buffer.spare_capacity_mut())
            .map_err(|e| Mp3Error::EncoderError(e))?;
        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded));
        }
        i += chunk_size;
    }
    Ok(())
}

fn get_num_channels(left: &Vec<f32>, right: &Vec<f32>) -> u8 {
    if left.len() > 0 && right.len() > 0 {
        2
    } else {
        1
    }
}
