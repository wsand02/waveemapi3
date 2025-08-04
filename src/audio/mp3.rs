use crate::error::Mp3Error;
use mp3lame_encoder::{BuildError, Builder, DualPcm, FlushNoGap, Id3Tag, MonoPcm};
use std::cmp;

pub fn mp3_encode(left: &Vec<f32>, right: &Vec<f32>) -> Result<Vec<u8>, Mp3Error> {
    let channels;
    let mut inputmono: Option<MonoPcm<f32>> = None;
    let mut inputdual: Option<DualPcm<f32>> = None;
    if left.len() > 0 && right.len() > 0 {
        inputdual = Some(DualPcm {
            left: &left[..],
            right: &right[..],
        });
        channels = 2;
    } else if left.len() > 0 {
        // incase of mono left is only ever decoded in wav_decode, thus dont need to check right
        inputmono = Some(MonoPcm(&left[..]));
        channels = 1;
    } else {
        return Err(Mp3Error::BuildError(BuildError::Generic));
    }
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
        .set_quality(mp3lame_encoder::Quality::Good)
        .map_err(|e| Mp3Error::BuildError(e))?;

    mp3_encoder
        .set_id3_tag(Id3Tag {
            title: b"My title",
            artist: &[],
            album_art: &[],
            album: b"My album",
            year: b"Current year",
            comment: b"Just my comment",
        })
        .map_err(|e| Mp3Error::Id3TagError(e))?;

    let mut mp3_encoder = mp3_encoder.build().map_err(|e| Mp3Error::BuildError(e))?;

    let mut mp3_out_buffer = Vec::new();
    let num_frames = cmp::max(left.len(), right.len());
    mp3_out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(num_frames));

    let mut encoded_size: Option<usize> = None;
    if Option::is_some(&inputdual) {
        encoded_size = Some(
            mp3_encoder
                .encode(inputdual.unwrap(), mp3_out_buffer.spare_capacity_mut())
                .map_err(|e| Mp3Error::EncoderError(e))?,
        );
    } else if Option::is_some(&inputmono) {
        encoded_size = Some(
            mp3_encoder
                .encode(inputmono.unwrap(), mp3_out_buffer.spare_capacity_mut())
                .map_err(|e| Mp3Error::EncoderError(e))?,
        );
    }
    if Option::is_none(&encoded_size) {
        return Err(Mp3Error::BuildError(BuildError::Generic));
    }

    unsafe {
        mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size.unwrap()));
    }

    let encoded_size = mp3_encoder
        .flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut())
        .map_err(|e| Mp3Error::EncoderError(e))?;
    unsafe {
        mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
    }
    Ok(mp3_out_buffer)
}
