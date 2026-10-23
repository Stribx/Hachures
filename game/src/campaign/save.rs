//! The campaign save: levels reached, sceaux, skills, choices and Names, and the player's settings.
//! One RON file in the user's data folder (localStorage in the browser), rewritten whole on every change.

use std::collections::BTreeMap;
#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::skills::Skill;
use crate::menu::Settings;

/// What a level left in the campaign.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Record {
    /// Sceau of the victory.
    pub won: bool,
    /// Sceau of the reading challenge.
    pub challenge: bool,
    /// Names kept in the Rôle on the last run.
    pub names: i32,
}

#[derive(Resource, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Save {
    /// Furthest level unlocked (index in `LEVELS`).
    pub reached: usize,
    /// One record per level, by index.
    pub levels: Vec<Record>,
    pub skills: Vec<Skill>,
    /// Choice id -> option picked, e.g. "refuge" -> "moulin".
    pub choices: BTreeMap<String, String>,
    pub settings: Settings,
}

impl Save {
    pub fn sceaux(&self) -> u32 {
        let earned: usize = self.levels.iter().map(|r| r.won as usize + r.challenge as usize).sum();
        // Saturating: a hand-edited save may have spent more than it earned.
        (earned as u32).saturating_sub(self.skills.iter().map(|s| s.cost()).sum())
    }

    /// The Rôle des Noms over the whole campaign.
    pub fn names(&self) -> i32 {
        self.levels.iter().map(|r| r.names).sum()
    }

    pub fn record(&mut self, level: usize) -> &mut Record {
        if self.levels.len() <= level {
            self.levels.resize(level + 1, Record::default());
        }
        &mut self.levels[level]
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load() -> Save {
        path().map(|p| Save::read(&p)).unwrap_or_default()
    }

    /// Writes the save, logging failures: losing a write must not stop the game.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn store(&self) {
        if let Some(path) = path()
            && let Err(err) = self.write(&path)
        {
            error!("could not save to {}: {err}", path.display());
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load() -> Save {
        let Some(storage) = web_storage() else { return Save::default() };
        let Some(text) = storage.get_item(WEB_KEY).ok().flatten() else { return Save::default() };
        ron::from_str(&text).unwrap_or_else(|err| {
            // Keep the unreadable save aside rather than overwrite it on the next write.
            warn!("unreadable save: {err}; kept as {WEB_KEY}.bak");
            let _ = storage.set_item(&format!("{WEB_KEY}.bak"), &text);
            Save::default()
        })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn store(&self) {
        let text = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default());
        match (web_storage(), text) {
            (Some(storage), Ok(text)) => {
                if storage.set_item(WEB_KEY, &text).is_err() {
                    error!("could not save: localStorage refused the write");
                }
            }
            _ => error!("could not save: no localStorage"),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn read(path: &Path) -> Save {
        let parsed = match fs::read_to_string(path) {
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Save::default(),
            Err(err) => Err(err.to_string()),
            Ok(text) => ron::from_str(&text).map_err(|err| err.to_string()),
        };
        parsed.unwrap_or_else(|err| {
            // Keep the unreadable file aside rather than overwrite it on the next write.
            let backup = path.with_extension("ron.bak");
            warn!("unreadable save {}: {err}; kept as {}", path.display(), backup.display());
            let _ = fs::rename(path, &backup);
            Save::default()
        })
    }

    /// Written to a temporary file first, then renamed: a crash mid-write leaves the old save whole.
    #[cfg(not(target_arch = "wasm32"))]
    fn write(&self, path: &Path) -> io::Result<()> {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let text = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).map_err(io::Error::other)?;
        let tmp = path.with_extension("ron.tmp");
        fs::write(&tmp, text)?;
        fs::rename(&tmp, path)
    }
}

/// `$XDG_DATA_HOME` or `~/.local/share` on Linux, `%APPDATA%` on Windows,
/// `~/Library/Application Support` on macOS.
#[cfg(not(target_arch = "wasm32"))]
fn path() -> Option<PathBuf> {
    let home = || std::env::var_os("HOME").map(PathBuf::from);
    let base = if cfg!(target_os = "windows") {
        std::env::var_os("APPDATA").map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        home().map(|h| h.join("Library/Application Support"))
    } else {
        std::env::var_os("XDG_DATA_HOME").map(PathBuf::from).or_else(|| home().map(|h| h.join(".local/share")))
    };
    Some(base?.join("hachures").join("save.ron"))
}

#[cfg(target_arch = "wasm32")]
const WEB_KEY: &str = "hachures-save";

#[cfg(target_arch = "wasm32")]
fn web_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_unreadable_save_kept() {
        let dir = std::env::temp_dir().join(format!("hachures-test-{}", std::process::id()));
        let path = dir.join("save.ron");
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(Save::read(&path), Save::default(), "no save yet");

        let mut save = Save { reached: 2, skills: vec![Skill::MineDePlomb], ..default() };
        *save.record(1) = Record { won: true, challenge: true, names: 3 };
        save.choices.insert("refuge".into(), "bois".into());
        save.settings = Settings { music: 40, effects: 0, fullscreen: true };
        save.write(&path).unwrap();
        assert_eq!(Save::read(&path), save);
        assert_eq!(save.sceaux(), 1, "2 earned, 1 spent");
        let older: Save = ron::from_str("(reached: 1)").unwrap();
        assert_eq!(older.settings, Settings::default(), "a save from before the settings loads with the defaults");

        fs::write(&path, "not a save").unwrap();
        assert_eq!(Save::read(&path), Save::default());
        assert_eq!(fs::read_to_string(dir.join("save.ron.bak")).unwrap(), "not a save", "kept aside, not lost");
        let _ = fs::remove_dir_all(&dir);
    }
}
