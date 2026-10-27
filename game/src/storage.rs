//! Small RON files kept for the player: one per `name` in the user's data folder (localStorage in the browser),
//! rewritten whole on every change.

#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use bevy::prelude::*;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// What `name` holds; none if it is missing, or unreadable (then kept aside, not overwritten by the next write).
#[cfg(not(target_arch = "wasm32"))]
pub fn load<T: DeserializeOwned>(name: &str) -> Option<T> {
    read(&path(name)?)
}

/// Writes `value` as `name`, logging failures: losing a write must not stop the game.
#[cfg(not(target_arch = "wasm32"))]
pub fn store<T: Serialize>(name: &str, value: &T) {
    if let Some(path) = path(name)
        && let Err(err) = write(&path, value)
    {
        error!("could not save to {}: {err}", path.display());
    }
}

#[cfg(target_arch = "wasm32")]
pub fn load<T: DeserializeOwned>(name: &str) -> Option<T> {
    let storage = web_storage()?;
    let key = format!("hachures-{name}");
    let text = storage.get_item(&key).ok().flatten()?;
    ron::from_str(&text)
        .inspect_err(|err| {
            warn!("unreadable {name}: {err}; kept as {key}.bak");
            let _ = storage.set_item(&format!("{key}.bak"), &text);
        })
        .ok()
}

#[cfg(target_arch = "wasm32")]
pub fn store<T: Serialize>(name: &str, value: &T) {
    let text = ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default());
    match (web_storage(), text) {
        (Some(storage), Ok(text)) => {
            if storage.set_item(&format!("hachures-{name}"), &text).is_err() {
                error!("could not save: localStorage refused the write");
            }
        }
        _ => error!("could not save: no localStorage"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn read<T: DeserializeOwned>(path: &Path) -> Option<T> {
    let parsed = match fs::read_to_string(path) {
        Err(err) if err.kind() == io::ErrorKind::NotFound => return None,
        Err(err) => Err(err.to_string()),
        Ok(text) => ron::from_str(&text).map_err(|err| err.to_string()),
    };
    parsed
        .inspect_err(|err| {
            // Keep the unreadable file aside rather than overwrite it on the next write.
            let backup = path.with_extension("ron.bak");
            warn!("unreadable {}: {err}; kept as {}", path.display(), backup.display());
            let _ = fs::rename(path, &backup);
        })
        .ok()
}

/// Written to a temporary file first, then renamed: a crash mid-write leaves the old file whole.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn write<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let text = ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default()).map_err(io::Error::other)?;
    let tmp = path.with_extension("ron.tmp");
    fs::write(&tmp, text)?;
    fs::rename(&tmp, path)
}

/// `$XDG_DATA_HOME` or `~/.local/share` on Linux, `%APPDATA%` on Windows,
/// `~/Library/Application Support` on macOS.
#[cfg(not(target_arch = "wasm32"))]
fn path(name: &str) -> Option<PathBuf> {
    let home = || std::env::var_os("HOME").map(PathBuf::from);
    let base = if cfg!(target_os = "windows") {
        std::env::var_os("APPDATA").map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        home().map(|h| h.join("Library/Application Support"))
    } else {
        std::env::var_os("XDG_DATA_HOME").map(PathBuf::from).or_else(|| home().map(|h| h.join(".local/share")))
    };
    Some(base?.join("hachures").join(format!("{name}.ron")))
}

#[cfg(target_arch = "wasm32")]
fn web_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}
