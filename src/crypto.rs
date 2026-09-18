use anyhow::Result;
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use serde::{de::DeserializeOwned, Serialize};
use std::io::{Read, Write};

pub trait Serializable: Serialize + DeserializeOwned + Sized {
    fn serialize(&self) -> Result<Vec<u8>> {
        Ok(postcard::to_allocvec(self)?)
    }

    fn deserialize(data: &[u8]) -> Result<Self> {
        Ok(postcard::from_bytes(data)?)
    }
}

pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(data);
    let mut output = Vec::new();
    decoder.read_to_end(&mut output)?;
    Ok(output)
}

pub fn is_binary(data: &[u8]) -> bool {
    data.contains(&0)
}
