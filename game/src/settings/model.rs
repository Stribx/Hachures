//! The `Settings` resource: every option the player can set, its file, and the quality presets.

use bevy::audio::Volume;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::storage;

#[derive(Resource, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Percents, in steps of 10.
    pub master: u32,
    pub music: u32,
    pub effects: u32,
    /// The land's sounds: wind, birds, rain, the river.
    pub ambient: u32,
    /// Silent while the window or the tab is in the background.
    pub mute_unfocused: bool,
    pub fullscreen: bool,
    pub vsync: bool,
    /// Percent of the window's pixels the board is drawn at, 50 to 100; the interface stays sharp.
    pub render_scale: u32,
    /// The performance overlay (`ui::perf`): 0 off, 1 the FPS, 2 detailed. F3 goes round them.
    pub performance: u8,
    /// The sun's shadow maps.
    pub shadows: bool,
    /// Ambient occlusion: 0 off, 1 low, 2 high (`look::occlusion`). Desktop only.
    pub occlusion: u8,
    /// 0 off, 1 FXAA, 2 SMAA. Desktop only: the browser keeps its MSAA.
    pub anti_aliasing: u8,
    pub depth_of_field: bool,
    /// The game pauses when the window or the tab loses focus.
    pub pause_unfocused: bool,
    /// Percent, 80 to 150.
    pub ui_scale: u32,
    /// Atkinson Hyperlegible instead of IM Fell.
    pub readable_font: bool,
    /// No pages rising, no buttons lifting, no depth of field.
    pub reduced_motion: bool,
    /// A code of `i18n::LANGUAGES`. Named `language`: saves of the first version kept a `lang`
    /// this one ignores.
    pub language: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            master: 100,
            music: 100,
            effects: 100,
            ambient: 70,
            mute_unfocused: true,
            fullscreen: false,
            vsync: true,
            render_scale: 100,
            performance: 0,
            shadows: true,
            occlusion: 1,
            anti_aliasing: 2,
            depth_of_field: true,
            pause_unfocused: true,
            ui_scale: 100,
            readable_font: false,
            reduced_motion: false,
            language: "fr".into(),
        }
    }
}

/// A save from before the settings had their own file: they were its `settings` field.
#[derive(Deserialize)]
struct Legacy {
    // The save wrote the bare struct, not `Some(..)`. Read loosely: settings it cannot make out
    // are lost, but the read never fails, so `storage` never sets the save aside over them.
    #[serde(default, deserialize_with = "loose")]
    settings: Option<Settings>,
}

fn loose<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<Settings>, D::Error> {
    Ok(ron::Value::deserialize(d).ok().and_then(|v| v.into_rust().ok()))
}

impl Settings {
    /// Its name in `storage`: `settings.ron`, or `hachures-settings` in the browser.
    pub const FILE: &str = "settings";

    /// From the settings file; else from the old save's field, else the defaults. Those two are
    /// written out at once.
    pub fn load() -> Settings {
        if let Some(settings) = storage::load(Settings::FILE) {
            return settings;
        }
        let settings = storage::load::<Legacy>(crate::campaign::save::Save::FILE)
            .and_then(|l| l.settings)
            .unwrap_or_default();
        settings.store();
        settings
    }

    pub fn store(&self) {
        storage::store(Settings::FILE, self);
    }

    /// Loudness is heard on a curve: half the slider is a quarter of the amplitude, clearly
    /// quieter.
    fn volume(&self, percent: u32) -> Volume {
        Volume::Linear((self.master as f32 / 100.0 * percent as f32 / 100.0).powi(2))
    }

    pub fn music_volume(&self, focused: bool) -> Volume {
        if focused || !self.mute_unfocused { self.volume(self.music) } else { Volume::SILENT }
    }

    /// The land's sounds, silent in the background like the music.
    pub fn ambient_volume(&self, focused: bool) -> Volume {
        if focused || !self.mute_unfocused { self.volume(self.ambient) } else { Volume::SILENT }
    }

    pub fn effects_volume(&self) -> Volume {
        self.volume(self.effects)
    }

    pub fn depth_of_field_shown(&self) -> bool {
        self.depth_of_field && !self.reduced_motion
    }

    /// The quality preset these graphics match, an index of `PRESETS`; none if the player tuned
    /// them by hand.
    pub(crate) fn preset(&self) -> Option<usize> {
        PRESETS.iter().position(|&p| {
            p == (self.shadows, self.occlusion, self.anti_aliasing, self.depth_of_field)
        })
    }
}

/// The quality presets, as an options page has them: shadows, ambient occlusion, anti-aliasing,
/// depth of field. High is the default. Measured with `HACHURES_FPS` at 2880x1800 on an integrated
/// GPU, the frame takes about 7 ms of GPU at Low, 14 at High and 17 at Ultra: occlusion in
/// `look::occlusion`, SMAA 1 ms, FXAA 0.85, the depth of field 1.5 and the shadows 1.
pub(crate) const PRESETS: [(bool, u8, u8, bool); 4] =
    [(false, 0, 1, false), (true, 0, 1, true), (true, 1, 2, true), (true, 2, 2, true)];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_follows_a_curve_and_the_focus() {
        let at =
            |master, music| Settings { master, music, ..default() }.music_volume(true).to_linear();
        assert_eq!(at(100, 0), 0.0);
        assert_eq!(at(100, 100), 1.0);
        assert_eq!(at(100, 50), 0.25);
        assert_eq!(at(50, 100), 0.25);
        assert_eq!(Settings::default().music_volume(false).to_linear(), 0.0);
        let unmuted = Settings { mute_unfocused: false, ..default() };
        assert_eq!(unmuted.music_volume(false).to_linear(), 1.0);
    }

    #[test]
    fn a_save_from_before_the_settings_file_carries_them() {
        let old: Legacy = ron::from_str("(reached: 1, settings: (music: 40))").unwrap();
        assert_eq!(old.settings, Some(Settings { music: 40, ..default() }));
        let older: Legacy = ron::from_str("(reached: 1)").unwrap();
        assert_eq!(older.settings, None, "a save from before the settings: the defaults");
        let odd: Legacy = ron::from_str("(reached: 1, settings: (music: \"loud\"))").unwrap();
        assert_eq!(
            odd.settings, None,
            "settings it cannot read are dropped, the save is still read"
        );
    }

    #[test]
    fn a_settings_file_from_before_the_overlay_has_it_off() {
        let old: Settings = ron::from_str("(music: 40)").unwrap();
        assert_eq!((old.music, old.performance), (40, 0));
        assert_eq!(Settings::default().performance, 0);
    }
}
