//! A setting of the settings page: its name, its value, and how a click or an arrow moves it.

use bevy::prelude::*;

use crate::i18n::{self, n_, t};
use crate::settings::{PRESETS, Settings};

const PRESET_NAMES: [&str; 5] = [
    n_("settings.preset.low"),
    n_("settings.preset.medium"),
    n_("settings.preset.high"),
    n_("settings.preset.ultra"),
    n_("settings.preset.custom"),
];

/// A setting: its row on the page.
#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub(super) enum Field {
    Master,
    Music,
    Effects,
    MuteUnfocused,
    PauseUnfocused,
    Lang,
    Fullscreen,
    Vsync,
    RenderScale,
    Preset,
    Shadows,
    Occlusion,
    AntiAliasing,
    Lens,
    UiScale,
    ReadableFont,
    ReducedMotion,
}

/// The page's groups, in two columns.
pub(super) const COLUMNS: [&[(&str, &[Field])]; 2] = [
    &[
        (n_("settings.group.sound"), &[Field::Master, Field::Music, Field::Effects, Field::MuteUnfocused]),
        (n_("settings.group.game"), &[Field::PauseUnfocused, Field::Lang]),
        (n_("settings.group.accessibility"), &[Field::UiScale, Field::ReadableFont, Field::ReducedMotion]),
    ],
    &[
        (n_("settings.group.display"), &[Field::Fullscreen, Field::Vsync, Field::RenderScale]),
        (
            n_("settings.group.graphics"),
            &[Field::Preset, Field::Shadows, Field::Occlusion, Field::AntiAliasing, Field::Lens],
        ),
    ],
];

/// Values of the choice settings.
const OCCLUSION: [&str; 3] = [n_("common.off"), n_("settings.preset.low"), n_("settings.preset.high")];
const ANTI_ALIASING: [&str; 3] = [n_("common.off"), n_("settings.aa.fxaa"), n_("settings.aa.smaa")];

impl Field {
    pub(super) fn name(self) -> &'static str {
        match self {
            Field::Master => n_("settings.master"),
            Field::Music => n_("settings.music"),
            Field::Effects => n_("settings.effects"),
            Field::MuteUnfocused => n_("settings.mute_unfocused"),
            Field::PauseUnfocused => n_("settings.pause_unfocused"),
            Field::Lang => n_("settings.language"),
            Field::Fullscreen => n_("settings.fullscreen"),
            Field::Vsync => n_("settings.vsync"),
            Field::RenderScale => n_("settings.render_scale"),
            Field::Preset => n_("settings.preset"),
            Field::Shadows => n_("settings.shadows"),
            Field::Occlusion => n_("settings.occlusion"),
            Field::AntiAliasing => n_("settings.anti_aliasing"),
            Field::Lens => n_("settings.depth_of_field"),
            Field::UiScale => n_("settings.ui_scale"),
            Field::ReadableFont => n_("settings.readable_font"),
            Field::ReducedMotion => n_("settings.reduced_motion"),
        }
    }

    /// A percent setting, and its bounds.
    fn range(self, s: &mut Settings) -> Option<(&mut u32, u32, u32)> {
        match self {
            Field::Master => Some((&mut s.master, 0, 100)),
            Field::Music => Some((&mut s.music, 0, 100)),
            Field::Effects => Some((&mut s.effects, 0, 100)),
            Field::RenderScale => Some((&mut s.render_scale, 50, 100)),
            Field::UiScale => Some((&mut s.ui_scale, 80, 150)),
            _ => None,
        }
    }

    /// An on/off setting.
    fn switch(self, s: &mut Settings) -> Option<&mut bool> {
        match self {
            Field::MuteUnfocused => Some(&mut s.mute_unfocused),
            Field::PauseUnfocused => Some(&mut s.pause_unfocused),
            Field::Fullscreen => Some(&mut s.fullscreen),
            Field::Vsync => Some(&mut s.vsync),
            Field::Shadows => Some(&mut s.shadows),
            Field::Lens => Some(&mut s.depth_of_field),
            Field::ReadableFont => Some(&mut s.readable_font),
            Field::ReducedMotion => Some(&mut s.reduced_motion),
            _ => None,
        }
    }

    /// A setting with a few named values: its index, and the values' keys.
    fn choice(self, s: &mut Settings) -> Option<(&mut u8, &'static [&'static str])> {
        match self {
            Field::Occlusion => Some((&mut s.occlusion, &OCCLUSION)),
            Field::AntiAliasing => Some((&mut s.anti_aliasing, &ANTI_ALIASING)),
            _ => None,
        }
    }

    pub(super) fn value(self, s: &Settings) -> String {
        let mut s = s.clone();
        if self == Field::Preset {
            t(PRESET_NAMES[s.preset().unwrap_or(PRESETS.len())]).to_string()
        } else if let Some((v, _, _)) = self.range(&mut s) {
            format!("{v} %")
        } else if let Some(on) = self.switch(&mut s) {
            if *on { t("common.on") } else { t("common.off") }.to_string()
        } else if let Some((i, names)) = self.choice(&mut s) {
            t(names[(*i as usize).min(names.len() - 1)]).to_string()
        } else {
            i18n::name(&s.language).to_string()
        }
    }

    /// ← (-1) or → (+1): a percent moves 10 within its bounds, a switch flips, a choice, a preset (from High when
    /// tuned by hand) or the language goes to the next, round.
    pub(super) fn step(self, s: &mut Settings, dir: i32) {
        let around = |i: usize, len: usize| (i as i32 + dir).rem_euclid(len as i32) as usize;
        if self == Field::Preset {
            let preset = PRESETS[around(s.preset().unwrap_or(2), PRESETS.len())];
            (s.shadows, s.occlusion, s.anti_aliasing, s.depth_of_field) = preset;
        } else if let Some((v, low, high)) = self.range(s) {
            *v = (*v as i32 + 10 * dir).clamp(low as i32, high as i32) as u32;
        } else if let Some(on) = self.switch(s) {
            *on = !*on;
        } else if let Some((i, names)) = self.choice(s) {
            *i = around(*i as usize, names.len()) as u8;
        } else {
            s.language = i18n::next(&s.language, dir).to_string();
        }
    }

    /// A click or Enter: the next value, the lowest after the highest.
    pub(super) fn next(self, s: &mut Settings) {
        match self.range(s) {
            Some((v, low, high)) => *v = if *v >= high { low } else { *v + 10 },
            None => self.step(s, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_stay_in_bounds_and_switches_flip() {
        let mut s = Settings::default();
        Field::Music.step(&mut s, 1);
        assert_eq!(s.music, 100);
        Field::UiScale.step(&mut s, -1);
        Field::UiScale.step(&mut s, -1);
        Field::UiScale.step(&mut s, -1);
        assert_eq!(s.ui_scale, 80);
        Field::Effects.next(&mut s);
        assert_eq!(s.effects, 0, "a click past the top starts again at the bottom");
        Field::Shadows.step(&mut s, -1);
        assert!(!s.shadows);
        Field::Lang.next(&mut s);
        assert_eq!(s.language, "en");
    }

    #[test]
    fn presets_set_the_graphics_and_a_hand_tuned_one_is_custom() {
        let mut s = Settings::default();
        assert_eq!(s.preset(), Some(2), "the default is High");
        Field::Preset.step(&mut s, 1);
        assert_eq!((s.preset(), s.occlusion), (Some(3), 2));
        Field::Preset.step(&mut s, 1);
        assert_eq!((s.preset(), s.shadows), (Some(0), false), "Ultra goes round to Low");
        Field::AntiAliasing.step(&mut s, -1);
        assert_eq!((s.preset(), s.anti_aliasing), (None, 0));
        assert_eq!(Field::AntiAliasing.value(&s), t("common.off"));
        Field::Preset.step(&mut s, -1);
        assert_eq!(s.preset(), Some(1), "from Custom, a step starts from High");
    }
}
