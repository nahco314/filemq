use crate::message::{find_earliest, read_earliest, read_message, write_message};
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::marker::PhantomData;
use std::path::PathBuf;
use uuid::Uuid;

pub fn read_message_req<T: DeserializeOwned>(space: &PathBuf, name: &str) -> Result<Option<T>> {
    read_message(space, "req", name)
}

pub struct ResponseHandler<T: DeserializeOwned> {
    space: PathBuf,
    name: String,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned> ResponseHandler<T> {
    pub fn new(space: PathBuf, name: String) -> Self {
        Self {
            space,
            name,
            _phantom: PhantomData,
        }
    }

    pub fn try_read(&self) -> Result<Option<T>> {
        read_message_res(&self.space, &self.name)
    }

    pub fn read(self) -> Result<T> {
        loop {
            if let Some(res) = self.try_read()? {
                return Ok(res);
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

pub fn write_message_req<T: Serialize, U: DeserializeOwned>(
    space: &PathBuf,
    message: T,
) -> Result<ResponseHandler<U>> {
    let name = Uuid::now_v7();

    write_message(space, "req", &name.to_string(), message)?;

    Ok(ResponseHandler::new(space.clone(), name.to_string()))
}

pub fn find_earliest_req(space: &PathBuf) -> Result<Option<String>> {
    find_earliest(space, "req")
}

pub fn read_earliest_req<T: DeserializeOwned>(space: &PathBuf) -> Result<Option<(String, T)>> {
    read_earliest(space, "req")
}

pub fn read_message_res<T: DeserializeOwned>(space: &PathBuf, name: &str) -> Result<Option<T>> {
    read_message(space, "res", name)
}

pub fn write_message_res<T: Serialize>(space: &PathBuf, name: &str, message: T) -> Result<()> {
    write_message(space, "res", name, message)
}
