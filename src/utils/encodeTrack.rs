use super::types::tracks::TrackInfo;
use base64::{engine::general_purpose, Engine as _};
use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

pub fn encode_track(track: &TrackInfo) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let version = if track.artwork_url.is_some() || track.isrc.is_some() {
        3
    } else if track.uri.is_some() {
        2
    } else {
        1
    };
    let is_versioned = if version > 1 { 1 } else { 0 };
    let first_int = is_versioned << 30;
    buffer.write_i32::<BigEndian>(first_int)?;
    if is_versioned == 1 {
        buffer.write_u8(version)?;
    }
    let write_utf = |buf: &mut Vec<u8>, s: &str| -> Result<(), std::io::Error> {
        let bytes = s.as_bytes();
        buf.write_u16::<BigEndian>(bytes.len() as u16)?;
        buf.write_all(bytes)?;
        Ok(())
    };
    let write_bool = |buf: &mut Vec<u8>, b: bool| -> Result<(), std::io::Error> {
        buf.write_u8(if b { 1 } else { 0 })?;
        Ok(())
    };
    write_utf(&mut buffer, &track.title)?;
    write_utf(&mut buffer, &track.author)?;
    buffer.write_i64::<BigEndian>(track.length)?;
    write_utf(&mut buffer, &track.identifier)?;
    write_bool(&mut buffer, track.is_stream)?;
    if version >= 2 {
        write_bool(&mut buffer, track.uri.is_some())?;
        if let Some(uri) = &track.uri {
            write_utf(&mut buffer, uri)?;
        }
    }
    if version == 3 {
        write_bool(&mut buffer, track.artwork_url.is_some())?;
        if let Some(artwork) = &track.artwork_url {
            write_utf(&mut buffer, artwork)?;
        }
        write_bool(&mut buffer, track.isrc.is_some())?;
        if let Some(isrc) = &track.isrc {
            write_utf(&mut buffer, isrc)?;
        }
    }
    write_utf(&mut buffer, &track.source_name)?;
    buffer.write_i64::<BigEndian>(track.position)?;
    Ok(general_purpose::STANDARD.encode(buffer))
}
