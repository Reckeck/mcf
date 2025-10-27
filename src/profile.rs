//! Module save/read/write video profile.
//!
//! TODO: Implement presets loader for video:
//! - Insert presets in finally project build.
//! - If need add new preset, add new file add build with preset.
//! - Support all popular formats.

use crate::geometry;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
/// Usage for later render and return producer.
pub enum RenderType {
    /// Detail information by [interlace video](https://en.wikipedia.org/wiki/Interlaced_video)
    /// Default for new profiles
    #[default]
    Interlace,
    /// Detail information by [progressive scan](https://en.wikipedia.org/wiki/Progressive_scan)
    Progressive,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ColorSpace {
    Rgb,
    #[default]
    None,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ExplicitType {
    #[default]
    Computed,
    Explicitly,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Ratio<Number, Denominator = Number> {
    pub number: Number,
    pub denominator: Denominator,
}

/// Structure keeping base properties video format.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Profile {
    pub frame_rate: Ratio<f32>,
    pub frame: geometry::Frame,
    pub sample_aspect: Ratio<f32>,
    pub display_aspect: Ratio<f32>,
    pub color_space: ColorSpace,
    pub explicit_type: ExplicitType,
}

impl Profile {
    /// Get the **video frame rate** as a floating point value.
    pub fn calculate_fps(&self) -> f32 {
        self.frame_rate.number / self.frame_rate.denominator
    }

    /// Get the **sample aspect ratio** as a floating point value.
    pub fn calculate_sar(&self) -> f32 {
        self.sample_aspect.number / self.sample_aspect.denominator
    }

    /// Get the **display aspect ratio** as floating point value.
    pub fn calculate_dar(&self) -> f32 {
        self.display_aspect.number / self.sample_aspect.denominator
    }
}

pub struct ProfileFactory(Profile);

impl ProfileFactory {
    pub fn new() -> Self {
        Self(Profile::default())
    }

    pub fn set_frame_rate(&mut self, frame_rate: Ratio<f32>) -> &Self {
        self.0.frame_rate = frame_rate;
        self
    }

    pub fn set_frame(&mut self, frame: geometry::Frame) -> &Self {
        self.0.frame = frame;
        self
    }

    pub fn set_sample_aspect(&mut self, sample_aspect: Ratio<f32>) -> &Self {
        self.0.sample_aspect = sample_aspect;
        self
    }

    pub fn set_display_aspect(&mut self, display_aspect: Ratio<f32>) -> &Self {
        self.0.display_aspect = display_aspect;
        self
    }

    pub fn set_color_space(&mut self, color_space: ColorSpace) -> &Self {
        self.0.color_space = color_space;
        self
    }

    pub fn set_explict_type(&mut self, explicit_type: ExplicitType) -> &Self {
        self.0.explicit_type = explicit_type;
        self
    }

    pub fn build(self) -> Profile {
        self.0
    }
}
