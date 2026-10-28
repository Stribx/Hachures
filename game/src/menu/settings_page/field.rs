//! A setting of the settings page: its name, its value, and how a click or an arrow moves it.

use bevy::prelude::*;

use crate::i18n::{self, n_, t, tf};
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
    Ambient,
    MuteUnfocused,
    PauseUnfocused,
    Language,
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
        (
            n_("settings.group.sound"),
            &[Field::Master, Field::Music, Field::Effects, Field::Ambient, Field::MuteUnfocused],
        ),
        (n_("settings.group.game"), &[Field::PauseUnfocused, Field::Language]),
        (
            n_("settings.group.accessibility"),
            &[Field::UiScale, Field::ReadableFont, Field::ReducedMotion],
        ),
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
const OCCLUSION: [&str; 3] =
    [n_("common.off"), n_("settings.preset.low"), n_("settings.preset.high")];
const ANTI_ALIASING: [&str; 3] = [n_("common.off"), n_("settings.aa.fxaa"), n_("settings.aa.smaa")];

impl Field {
    pub(super) fn name(self) -> &'static str {
        match self {
            Field::Master => n_("settings.master"),
            Field::Music => n_("settings.music"),
            Field::Effects => n_("settings.effects"),
            Field::Ambient => n_("settings.ambient"),
            Field::MuteUnfocused => n_("settings.mute_unfocused"),
            Field::PauseUnfocused => n_("settings.pause_unfocused"),
            Field::Language => n_("settings.language"),
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
    fn range(self, settings: &mut Settings) -> Option<(&mut u32, u32, u32)> {
        match self {
            Field::Master => Some((&mut settings.master, 0, 100)),
            Field::Music => Some((&mut settings.music, 0, 100)),
            Field::Effects => Some((&mut settings.effects, 0, 100)),
            Field::Ambient => Some((&mut settings.ambient, 0, 100)),
            Field::RenderScale => Some((&mut settings.render_scale, 50, 100)),
            Field::UiScale => Some((&mut settings.ui_scale, 80, 150)),
            _ => None,
        }
    }

    /// An on/off setting.
    fn switch(self, settings: &mut Settings) -> Option<&mut bool> {
        match self {
            Field::MuteUnfocused => Some(&mut settings.mute_unfocused),
            Field::PauseUnfocused => Some(&mut settings.pause_unfocused),
            Field::Fullscreen => Some(&mut settings.fullscreen),
            Field::Vsync => Some(&mut settings.vsync),
            Field::Shadows => Some(&mut settings.shadows),
            Field::Lens => Some(&mut settings.depth_of_field),
            Field::ReadableFont => Some(&mut settings.readable_font),
            Field::ReducedMotion => Some(&mut settings.reduced_motion),
            _ => None,
        }
    }

    /// A setting with a few named values: its index, and the values' keys.
    fn choice(self, settings: &mut Settings) -> Option<(&mut u8, &'static [&'static str])> {
        match self {
            Field::Occlusion => Some((&mut settings.occlusion, &OCCLUSION)),
            Field::AntiAliasing => Some((&mut settings.anti_aliasing, &ANTI_ALIASING)),
            _ => None,
        }
    }

    pub(super) fn value(self, settings: &Settings) -> String {
        let mut settings = settings.clone();
        if self == Field::Preset {
            t(PRESET_NAMES[settings.preset().unwrap_or(PRESETS.len())]).to_string()
        } else if let Some((v, _, _)) = self.range(&mut settings) {
            tf("settings.percent", &[&v])
        } else if let Some(on) = self.switch(&mut settings) {
            if *on { t("common.on") } else { t("common.off") }.to_string()
        } else if let Some((i, names)) = self.choice(&mut settings) {
            t(names[(*i as usize).min(names.len() - 1)]).to_string()
        } else {
            i18n::name(&settings.language).to_string()
        }
    }

    /// ← (-1) or → (+1): a percent moves 10 within its bounds, a switch flips, a choice, a preset (from High when
    /// tuned by hand) or the language goes to the next, round.
    pub(super) fn step(self, settings: &mut Settings, direction: i32) {
        let around = |i: usize, len: usize| (i as i32 + direction).rem_euclid(len as i32) as usize;
        if self == Field::Preset {
            let preset = PRESETS[around(settings.preset().unwrap_or(2), PRESETS.len())];
            (
                settings.shadows,
                settings.occlusion,
                settings.anti_aliasing,
                settings.depth_of_field,
            ) = preset;
        } else if let Some((v, low, high)) = self.range(settings) {
            *v = (*v as i32 + 10 * direction).clamp(low as i32, high as i32) as u32;
        } else if let Some(on) = self.switch(settings) {
            *on = !*on;
        } else if let Some((i, names)) = self.choice(settings) {
            *i = around(*i as usize, names.len()) as u8;
        } else {
            settings.language = i18n::next(&settings.language, direction).to_string();
        }
    }

    /// A click or Enter: the next value, the lowest after the highest.
    pub(super) fn advance(self, settings: &mut Settings) {
        match self.range(settings) {
            Some((v, low, high)) => *v = if *v >= high { low } else { *v + 10 },
            None => self.step(settings, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_stay_in_bounds_and_switches_flip() {
        let mut settings = Settings::default();
        Field::Music.step(&mut settings, 1);
        assert_eq!(settings.music, 100);
        Field::UiScale.step(&mut settings, -1);
        Field::UiScale.step(&mut settings, -1);
        Field::UiScale.step(&mut settings, -1);
        assert_eq!(settings.ui_scale, 80);
        Field::Effects.advance(&mut settings);
        assert_eq!(settings.effects, 0, "a click past the top starts again at the bottom");
        Field::Shadows.step(&mut settings, -1);
        assert!(!settings.shadows);
        Field::Language.advance(&mut settings);
        assert_eq!(settings.language, "en");
    }

    #[test]
    fn presets_set_the_graphics_and_a_hand_tuned_one_is_custom() {
        let mut settings = Settings::default();
        assert_eq!(settings.preset(), Some(2), "the default is High");
        Field::Preset.step(&mut settings, 1);
        assert_eq!((settings.preset(), settings.occlusion), (Some(3), 2));
        Field::Preset.step(&mut settings, 1);
        assert_eq!(
            (settings.preset(), settings.shadows),
            (Some(0), false),
            "Ultra goes round to Low"
        );
        Field::AntiAliasing.step(&mut settings, -1);
        assert_eq!((settings.preset(), settings.anti_aliasing), (None, 0));
        assert_eq!(Field::AntiAliasing.value(&settings), t("common.off"));
        Field::Preset.step(&mut settings, -1);
        assert_eq!(settings.preset(), Some(1), "from Custom, a step starts from High");
    }
}
