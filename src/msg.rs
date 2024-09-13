use crate::message::{find_earliest, read_earliest, read_message, write_message};
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

pub fn read_message_msg<T: DeserializeOwned>(space: &PathBuf, name: &str) -> Result<Option<T>> {
    read_message(space, "msg", name)
}

pub fn write_message_msg<T: Serialize>(space: &PathBuf, message: T) -> Result<()> {
    let name = Uuid::now_v7();

    write_message(space, "msg", &name.to_string(), message)
}

pub fn find_earliest_msg(space: &PathBuf) -> Result<Option<String>> {
    find_earliest(space, "msg")
}

pub fn read_earliest_msg<T: DeserializeOwned>(space: &PathBuf) -> Result<Option<(String, T)>> {
    read_earliest(space, "msg")
}
