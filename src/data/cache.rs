use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fmt, fs};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::de::StdError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct One(HashMap<String, String>);

#[derive(Debug)]
pub enum CacheError {
    IoError(std::io::Error),
    SerializationError(bincode::Error),
    CacheExpired,
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::IoError(e) => write!(f, "IO error: {}", e),
            CacheError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            CacheError::CacheExpired => write!(f, "Cache has expired"),
        }
    }
}
impl StdError for CacheError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            CacheError::IoError(e) => Some(e),
            CacheError::SerializationError(e) => Some(e),
            CacheError::CacheExpired => None,
        }
    }
}

impl From<std::io::Error> for CacheError {
    fn from(err: std::io::Error) -> Self {
        CacheError::IoError(err)
    }
}

impl From<bincode::Error> for CacheError {
    fn from(err: bincode::Error) -> Self {
        CacheError::SerializationError(err)
    }
}

pub struct CacheManager {
    cache_duration: u64,
}

impl CacheManager {
    pub fn new(cache_duration: u64) -> Self {
        Self { cache_duration }
    }

    pub(crate) fn load_cache(&self, filename: &str) -> Result<One, CacheError> {
        let data = fs::read(filename)?;
        let (_timestamp, cached_data): (u64, Vec<u8>) = bincode::deserialize(&data)?;


        let deserialized: One = bincode::deserialize(&cached_data)?;
        Ok(deserialized)
    }


    fn save_cache(&self, data: &One, filename: &str) -> Result<(), CacheError> {
        let serialized = bincode::serialize(&data)?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let cache_data = (timestamp, serialized);
        let cache_serialized = bincode::serialize(&cache_data)?;

        fs::write(filename, cache_serialized)?;
        Ok(())
    }
}

pub struct H9123 {
    pub first_list: One,
    pub second_list: One,
}

impl H9123 {
    pub fn new(first_list: One, second_list: One) -> Self {
        Self {
            first_list,
            second_list,
        }
    }

    pub fn search_standard(&self, ids: &[String]) -> Vec<(String, Option<String>, Option<String>)> {
        ids.iter()
            .map(|id| {
                let first_value = self.first_list.0.get(id).cloned();
                let second_value = self.second_list.0.get(id).cloned();
                (id.clone(), first_value, second_value)
            })
            .collect()
    }

}



