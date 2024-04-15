use lzzzz::lz4;
use serde::{de::DeserializeOwned, Serialize};

pub trait BinaryBlob: DeserializeOwned + Serialize {
    fn from_option_blob(bytes: Option<&[u8]>) -> anyhow::Result<Option<Self>> {
        bytes.map(|b| Self::from_blob(b)).transpose()
    }

    fn from_blob(bytes: &[u8]) -> anyhow::Result<Self> {
        let len = u16::from_le_bytes([bytes[0], bytes[1]]) as usize;
        let mut buf = vec![0; len];
        let comp = &bytes[2..];
        lz4::decompress(comp, &mut buf)?;
        Ok(bincode::deserialize(&buf)?)
    }

    fn to_blob(&self) -> anyhow::Result<Vec<u8>> {
        let data = bincode::serialize(self)?;

        let max = lz4::max_compressed_size(data.len());
        let mut comp = vec![0; max + 2];
        // Write the len as u16
        comp[..2].copy_from_slice(&(data.len() as u16).to_le_bytes());
        // Compress the data
        let len = lz4::compress(&data, &mut comp[2..], lz4::ACC_LEVEL_DEFAULT)?;
        comp.truncate(len + 2);
        Ok(comp)
    }
}

impl BinaryBlob for String {}
impl BinaryBlob for usize {}
impl<T: BinaryBlob> BinaryBlob for Option<T> {}
impl<T: BinaryBlob> BinaryBlob for Vec<T> {}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_blob<T: BinaryBlob + PartialEq + Eq + std::fmt::Debug>(data: T) {
        let blob = data.to_blob().unwrap();
        let data2 = T::from_blob(&blob).unwrap();
        assert_eq!(data, data2);
    }

    #[test]
    fn blob_test() {
        test_blob("Hello World".to_string());
        test_blob(vec![String::from("Hello World"); 200]);
    }
}
