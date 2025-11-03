//! Module save/read/write video profile.
//!
//! TODO: Implement presets loader for video:
//! - Insert presets in finally project build.
//! - If need add new preset, add new file add build with preset.
//! - Support all popular formats.

use serde::{Deserialize, Serialize};

use crate::{color::spaces::ColorSpace, geometry};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
/// Usage for later render and return producer.
pub enum RenderType {
    /// Detail information by [interlace video](https://en.wikipedia.org/wiki/Interlaced_video)
    /// Default for new profiles
    Interlace,
    /// Detail information by [progressive scan](https://en.wikipedia.org/wiki/Progressive_scan)
    #[default]
    Progressive,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub enum ExplicitType {
    #[default]
    Computed,
    Explicitly,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Ratio<Number, Denominator = Number> {
    pub number: Number,
    pub denominator: Denominator,
}

/// Structure keeping base properties video format.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Profile {
    pub frame: geometry::Frame,
    pub frame_rate: Ratio<f32>,
    pub sample_aspect: Ratio<f32>,
    pub display_aspect: Ratio<f32>,
    pub color_space: ColorSpace,
    pub explicit_type: ExplicitType,
    pub render_type: RenderType,
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

pub struct ProfileBuilder(Profile);

impl ProfileBuilder {
    pub fn new() -> Self {
        Self(Profile::default())
    }

    pub fn set_frame_rate(&mut self, frame_rate: Ratio<f32>) -> &mut Self {
        self.0.frame_rate = frame_rate;
        self
    }

    pub fn set_frame(&mut self, frame: geometry::Frame) -> &mut Self {
        self.0.frame = frame;
        self
    }

    pub fn set_sample_aspect(&mut self, sample_aspect: Ratio<f32>) -> &mut Self {
        self.0.sample_aspect = sample_aspect;
        self
    }

    pub fn set_display_aspect(&mut self, display_aspect: Ratio<f32>) -> &mut Self {
        self.0.display_aspect = display_aspect;
        self
    }

    pub fn set_color_space(&mut self, color_space: ColorSpace) -> &mut Self {
        self.0.color_space = color_space;
        self
    }

    pub fn set_explicit_type(&mut self, explicit_type: ExplicitType) -> &mut Self {
        self.0.explicit_type = explicit_type;
        self
    }

    pub fn set_render_type(&mut self, render_type: RenderType) -> &mut Self {
        self.0.render_type = render_type;
        self
    }

    pub fn build(&self) -> Profile {
        self.0.to_owned()
    }
}

include!("./generate/profiles.rs");
