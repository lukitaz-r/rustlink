use super::types::tracks::{DecodedTrack, TrackInfo};
use base64::{engine::general_purpose, Engine as _};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

pub fn decode_track(encoded: &str) -> Result<DecodedTrack, Box<dyn std::error::Error>> {
    let decoded_bytes = general_purpose::STANDARD.decode(encoded)?;
    let mut reader = Cursor::new(decoded_bytes);
    let first_int = reader.read_i32::<BigEndian>()?;
    let is_versioned = ((first_int & (-1073741824)) >> 30) & 1;
    let version = if is_versioned != 0 {
        reader.read_u8()?
    } else {
        1
    };
    let read_utf = |reader: &mut Cursor<Vec<u8>>| -> Result<String, std::io::Error> {
        let len = reader.read_u16::<BigEndian>()?;
        let mut buf = vec![0u8; len as usize];
        reader.read_exact(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    };
    let read_bool = |reader: &mut Cursor<Vec<u8>>| -> Result<bool, std::io::Error> {
        Ok(reader.read_u8()? != 0)
    };
    let title = read_utf(&mut reader)?;
    let author = read_utf(&mut reader)?;
    let length = reader.read_i64::<BigEndian>()?;
    let identifier = read_utf(&mut reader)?;
    let is_stream = read_bool(&mut reader)?;

    let uri = if version >= 2 && read_bool(&mut reader)? {
        Some(read_utf(&mut reader)?)
    } else {
        None
    };
    let artwork_url = if version == 3 && read_bool(&mut reader)? {
        Some(read_utf(&mut reader)?)
    } else {
        None
    };
    let isrc = if version == 3 && read_bool(&mut reader)? {
        Some(read_utf(&mut reader)?)
    } else {
        None
    };
    let source_name = read_utf(&mut reader)?;
    let position = reader.read_i64::<BigEndian>()?;
    Ok(DecodedTrack {
        encoded: encoded.to_string(),
        info: TrackInfo {
            title,
            author,
            length,
            identifier,
            is_seekable: true,
            is_stream,
            uri,
            artwork_url,
            isrc,
            source_name,
            position,
        },
    })
}
