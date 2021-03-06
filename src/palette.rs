use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette {
    base_lightness_range: Range<f32>,
    low_lightness: f32,
    high_lightness: f32,
    low_chroma: f32,
    medium_chroma: f32,
    high_chroma: f32,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            base_lightness_range: 0.17..1.0,
            low_lightness: 0.8,
            high_lightness: 0.9,
            low_chroma: 0.032,
            medium_chroma: 0.07,
            high_chroma: 0.1,
        }
    }
}

impl Palette {
    pub(crate) fn chroma() -> Self {
        Self {
            low_chroma: 0.06,
            medium_chroma: 0.09,
            high_chroma: 0.11,
            high_lightness: 0.86, // allows pushing chroma a little higher
            ..Self::default()
        }
    }

    pub(crate) fn soft() -> Self {
        Self {
            base_lightness_range: 0.25..0.95,
            ..Self::default()
        }
    }

    pub(crate) fn soft_chroma() -> Self {
        Self {
            base_lightness_range: 0.25..0.95,
            ..Self::chroma()
        }
    }

    pub(crate) fn light() -> Self {
        Self {
            base_lightness_range: 1.0..0.2,
            low_lightness: 0.65,
            high_lightness: 0.55,
            low_chroma: 0.04,
            medium_chroma: 0.06,
            high_chroma: 0.08,
        }
    }

    pub(crate) fn light_chroma() -> Self {
        Self {
            low_chroma: 0.09,
            medium_chroma: 0.1,
            high_chroma: 0.12,
            ..Self::light()
        }
    }

    pub(crate) fn light_soft() -> Self {
        Self {
            base_lightness_range: 0.96..0.3,
            ..Self::light()
        }
    }

    pub(crate) fn light_soft_chroma() -> Self {
        Self {
            base_lightness_range: 0.96..0.3,
            ..Self::light_chroma()
        }
    }

    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(
            lerp(scale.value(), self.base_lightness_range.clone()),
            0.0,
            0.0,
        )
    }

    pub(crate) fn pink(&self) -> Oklch {
        oklch(self.high_lightness, self.low_chroma, 0.0)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(self.low_lightness, self.high_chroma, 30.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(self.high_lightness, self.low_chroma, 105.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(self.high_lightness, self.medium_chroma, 130.0)
    }

    pub(crate) fn light_green(&self) -> Oklch {
        oklch(self.high_lightness, self.low_chroma, 130.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(self.low_lightness, self.high_chroma, 230.0)
    }

    pub(crate) fn light_blue(&self) -> Oklch {
        oklch(self.high_lightness, self.low_chroma, 240.0)
    }

    pub(crate) fn lavender(&self) -> Oklch {
        oklch(self.high_lightness, self.low_chroma, 285.0)
    }

    pub(crate) fn magenta(&self) -> Oklch {
        oklch(self.low_lightness, self.high_chroma, 330.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    LightBg,
    LighterBg,
    DarkFg,
    DimFg,
    Fg,
    BrightFg,
}

impl BaseScale {
    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::LightBg => 0.1,
            Self::LighterBg => 0.25,
            Self::DarkFg => 0.35,
            Self::DimFg => 0.6,
            Self::Fg => 0.85,
            Self::BrightFg => 1.0,
        }
    }
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
