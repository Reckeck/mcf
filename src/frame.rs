use serde::{Deserialize, Serialize};

use crate::{geometry, profile::Profile, structures::properties};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct FrameMetaData {}

#[derive(Debug, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Frame {
    pub profile: Profile,
    pub aspect_ratio: f32,
    pub viewport: geometry::Frame,
    pub properties: properties::Properties,
    pub meta: FrameMetaData,

    position: geometry::Position,
    speed: i8,
}

pub struct FrameBuilder(Frame);

impl FrameBuilder {
    pub fn new() -> Self {
        Self(Frame::default())
    }

    pub fn set_profile(&mut self, profile: &Profile) -> &Self {
        self.0.profile = profile.clone();
        self
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: geometry::Coordinate) -> &Self {
        self.0.aspect_ratio = aspect_ratio;
        self
    }

    pub fn set_viewport(&mut self, viewport: geometry::Frame) -> &Self {
        self.0.viewport = viewport;
        self
    }

    pub fn set_properties(&mut self, properties: properties::Properties) -> &Self {
        self.0.properties = properties;
        self
    }

    pub fn set_meta(&mut self, meta: FrameMetaData) -> &Self {
        self.0.meta = meta;
        self
    }

    pub fn set_position(&mut self, position: geometry::Position) -> &Self {
        self.0.position = position;
        self
    }

    pub fn set_speed(&mut self, speed: i8) -> &Self {
        self.0.speed = speed;
        self
    }
}

mod ffi {
    use super::*;

    #[repr(C)]
    pub enum COption<T> {
        Some(T),
        None,
    }

    mod getters {
        use super::*;

        #[unsafe(no_mangle)]
        pub extern "C" fn frame_get_profile(frame: *const Frame) -> COption<Profile> {
            if frame.is_null() {
                return COption::None;
            }

            COption::Some(unsafe { (*frame).profile.clone() })
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn frame_get_aspect_ratio(
            frame: *const Frame,
        ) -> COption<geometry::Coordinate> {
            if frame.is_null() {
                return COption::None;
            }

            COption::Some(unsafe { (*frame).aspect_ratio.clone() })
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn frame_get_viewport(frame: *const Frame) -> COption<geometry::Frame> {
            if frame.is_null() {
                return COption::None;
            }

            COption::Some(unsafe { (*frame).viewport.clone() })
        }

        // #[unsafe(no_mangle)]
        // pub extern "C" fn frame_get_meta(frame: *const Frame) -> COption<FrameMetaData> {
        //     if frame.is_null() {
        //         return COption::None;
        //     }

        //     COption::Some(unsafe { (*frame).meta.clone() })
        // }

        #[unsafe(no_mangle)]
        pub extern "C" fn frame_get_position(frame: *const Frame) -> COption<geometry::Position> {
            if frame.is_null() {
                return COption::None;
            }

            COption::Some(unsafe { (*frame).position.clone() })
        }
    }

    mod setters {
        use super::*;

        #[unsafe(no_mangle)]
        pub extern "C" fn frame_set_profile(frame: *mut Frame, profile: Profile) {
            if !frame.is_null() {
                unsafe { (*frame).profile = profile };
            }
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn position_set_aspect_ratio(
            frame: *mut Frame,
            aspect_ratio: geometry::Coordinate,
        ) {
            if !frame.is_null() {
                unsafe { (*frame).aspect_ratio = aspect_ratio };
            }
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn position_set_viewport(frame: *mut Frame, viewport: geometry::Frame) {
            if !frame.is_null() {
                unsafe { (*frame).viewport = viewport };
            }
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn position_set_speed(frame: *mut Frame, speed: i8) {
            if !frame.is_null() {
                unsafe { (*frame).speed = speed };
            }
        }
    }
}
