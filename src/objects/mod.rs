pub mod fileedit;
pub mod commit;

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

pub trait Serializable: Serialize + DeserializeOwned + Sized {
    fn serialize(&self) -> Result<Vec<u8>> {
        Ok(postcard::to_allocvec(self)?)
    }

    fn deserialize(data: &[u8]) -> Result<Self> {
        Ok(postcard::from_bytes(data)?)
    }
}
