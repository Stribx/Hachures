//! What a frame costs, from the diagnostics: its time, and the GPU's share of it, the sum of the
//! render passes' timings. The performance overlay shows them; the dynamic resolution follows them.

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

/// The sum of the top-level render passes' GPU times (`render/<pass>/elapsed_gpu`): a nested pass
/// is already inside its parent's. None when no pass is timed (WebGL2, some drivers).
pub fn gpu_ms<'a>(diagnostics: impl Iterator<Item = (&'a str, f64)>) -> Option<f64> {
    let top: Vec<f64> = diagnostics
        .filter(|(path, _)| {
            matches!(path.split('/').collect::<Vec<_>>()[..], ["render", _, "elapsed_gpu"])
        })
        .map(|(_, ms)| ms)
        .collect();
    (!top.is_empty()).then(|| top.iter().sum())
}

/// The GPU's time a frame, smoothed, in milliseconds.
pub fn gpu_time(store: &DiagnosticsStore) -> Option<f64> {
    gpu_ms(store.iter().filter_map(|d| Some((d.path().as_str(), d.smoothed()?))))
}

/// A frame's time, smoothed, in milliseconds.
pub fn frame_time(store: &DiagnosticsStore) -> Option<f64> {
    store.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME).and_then(|d| d.smoothed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_gpu_time_is_the_sum_of_the_top_level_passes() {
        let passes = [
            ("render/ssao/elapsed_gpu", 1.5),
            ("render/fxaa/elapsed_gpu", 0.8),
            // Inside fxaa: already in its time.
            ("render/fxaa/fxaa/elapsed_gpu", 0.8),
            ("render/ssao/elapsed_cpu", 9.0),
            ("fps", 60.0),
        ];
        let gpu = gpu_ms(passes.into_iter()).unwrap();
        assert!((gpu - 2.3).abs() < 1e-9, "{gpu}");
        assert_eq!(gpu_ms([("fps", 60.0)].into_iter()), None, "no timing, as in a browser");
    }
}
