use crate::msg::read_message_msg;
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub(crate) fn read_message<T: DeserializeOwned>(
    space: &PathBuf,
    kind: &str,
    name: &str,
) -> Result<Option<T>> {
    let mut message_path = space.clone();
    message_path.push(kind);
    message_path.push(name);
    let mut wip_path = space.clone();
    wip_path.push("wip");
    fs::DirBuilder::new().recursive(true).create(&wip_path)?;
    wip_path.push(name);

    match fs::rename(&message_path, &wip_path) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.into()),
    };

    let message_string = fs::read_to_string(&wip_path)?;
    let message = serde_json::from_str::<T>(&message_string)?;

    fs::remove_file(&wip_path)?;

    Ok(Some(message))
}

pub(crate) fn write_message<T: Serialize>(
    space: &PathBuf,
    kind: &str,
    name: &str,
    message: T,
) -> Result<()> {
    let mut message_path = space.clone();
    message_path.push(kind);
    fs::DirBuilder::new()
        .recursive(true)
        .create(&message_path)?;
    message_path.push(name);
    let mut wip_path = space.clone();
    wip_path.push("wip");
    fs::DirBuilder::new().recursive(true).create(&wip_path)?;
    wip_path.push(name);

    let message_string = serde_json::to_string(&message)?;

    fs::write(&wip_path, message_string)?;
    fs::rename(&wip_path, &message_path)?;

    Ok(())
}

pub(crate) fn find_earliest(space: &PathBuf, kind: &str) -> Result<Option<String>> {
    let mut path = space.clone();
    path.push(kind);

    if !path.exists() {
        return Ok(None);
    }

    let mut earliest = None;

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let name = name.to_string();

        if let Some(v) = &earliest {
            if &name < v {
                earliest = Some(name.to_string());
            }
        } else {
            earliest = Some(name.to_string());
        }
    }

    Ok(earliest)
}

pub(crate) fn read_earliest<T: DeserializeOwned>(
    space: &PathBuf,
    kind: &str,
) -> Result<Option<(String, T)>> {
    if let Some(name) = find_earliest(space, kind)? {
        if let Some(message) = read_message(space, kind, &name)? {
            return Ok(Some((name, message)));
        }
    }

    Ok(None)
}
