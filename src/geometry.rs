use serde::{Deserialize, Serialize};

pub type Coordinate = f32;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Position {
    pub x: Coordinate,
    pub y: Coordinate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Rect {
    pub top: Coordinate,
    pub left: Coordinate,
    pub right: Coordinate,
    pub bottom: Coordinate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Frame {
    pub width: Coordinate,
    pub height: Coordinate,
}

impl Frame {
    /// Calculate the width scale factor.
    pub(crate) fn calculate_scale_width(&self, width: Coordinate) -> Coordinate {
        width / self.width
    }

    /// Get the height scale factor.
    pub(crate) fn calculate_scale_height(&self, height: Coordinate) -> Coordinate {
        height / self.height
    }
}

mod ffi {
    use super::*;

    mod position {
        use super::*;

        #[unsafe(no_mangle)]
        pub extern "C" fn position_new(x: Coordinate, y: Coordinate) -> *mut Position {
            Box::into_raw(Box::new(Position { x, y }))
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn position_destroy(position: *mut Position) {
            if !position.is_null() {
                unsafe {
                    let _ = Box::from_raw(position);
                };
            }
        }

        pub mod getters {
            use super::*;

            #[unsafe(no_mangle)]
            pub extern "C" fn position_get_x(position: *const Position) -> Coordinate {
                if position.is_null() {
                    return 0.0;
                }
                unsafe { (*position).x }
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn position_get_y(position: *const Position) -> Coordinate {
                if position.is_null() {
                    return 0.0;
                }
                unsafe { (*position).y }
            }
        }

        pub mod setters {
            use super::*;

            #[unsafe(no_mangle)]
            pub extern "C" fn position_set_x(position: *mut Position, x: Coordinate) {
                if !position.is_null() {
                    unsafe { (*position).x = x };
                }
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn position_set_y(position: *mut Position, y: Coordinate) {
                if !position.is_null() {
                    unsafe { (*position).y = y };
                }
            }
        }
    }

    mod rect {
        use super::*;

        #[unsafe(no_mangle)]
        pub extern "C" fn rect_new(
            top: Coordinate,
            left: Coordinate,
            right: Coordinate,
            bottom: Coordinate,
        ) -> *mut Rect {
            Box::into_raw(Box::new(Rect {
                top,
                left,
                right,
                bottom,
            }))
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn rect_destroy(rect: *mut Rect) {
            if !rect.is_null() {
                unsafe {
                    let _ = Box::from_raw(rect);
                };
            }
        }

        mod getters {
            use super::*;

            #[unsafe(no_mangle)]
            pub extern "C" fn rect_get_top(rect: *const Rect) -> Coordinate {
                if rect.is_null() {
                    return 0.0;
                }
                unsafe { (*rect).top }
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn rect_get_bottom(rect: *const Rect) -> Coordinate {
                if rect.is_null() {
                    return 0.0;
                }
                unsafe { (*rect).bottom }
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn rect_get_left(rect: *const Rect) -> Coordinate {
                if rect.is_null() {
                    return 0.0;
                }
                unsafe { (*rect).left }
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn rect_get_right(rect: *const Rect) -> Coordinate {
                if rect.is_null() {
                    return 0.0;
                }
                unsafe { (*rect).right }
            }
        }

        mod setters {
            use super::*;

            #[unsafe(no_mangle)]
            pub extern "C" fn rect_set_top(rect: *mut Rect, top: Coordinate) {
                if !rect.is_null() {
                    unsafe { (*rect).top = top };
                }
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn rect_set_bottom(rect: *mut Rect, bottom: Coordinate) {
                if !rect.is_null() {
                    unsafe { (*rect).bottom = bottom };
                }
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn rect_set_left(rect: *mut Rect, left: Coordinate) {
                if !rect.is_null() {
                    unsafe { (*rect).left = left };
                }
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn rect_set_right(rect: *mut Rect, right: Coordinate) {
                if !rect.is_null() {
                    unsafe { (*rect).right = right };
                }
            }
        }
    }

    mod frame {
        use super::*;

        #[unsafe(no_mangle)]
        pub extern "C" fn frame_new(width: Coordinate, height: Coordinate) -> *mut Frame {
            Box::into_raw(Box::new(Frame { width, height }))
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn frame_destroy(frame: *mut Frame) {
            if !frame.is_null() {
                unsafe {
                    let _ = Box::from_raw(frame);
                };
            }
        }

        mod getters {
            use super::*;

            #[unsafe(no_mangle)]
            pub extern "C" fn frame_get_width(frame: *const Frame) -> Coordinate {
                if frame.is_null() {
                    return 0.0;
                }
                unsafe { (*frame).width }
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn frame_get_height(frame: *const Frame) -> Coordinate {
                if frame.is_null() {
                    return 0.0;
                }
                unsafe { (*frame).height }
            }
        }

        mod setters {
            use super::*;

            #[unsafe(no_mangle)]
            pub extern "C" fn frame_set_width(frame: *mut Frame, width: Coordinate) {
                if !frame.is_null() {
                    unsafe { (*frame).width = width };
                }
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn frame_set_height(frame: *mut Frame, height: Coordinate) {
                if !frame.is_null() {
                    unsafe { (*frame).height = height };
                }
            }
        }

        mod calculate {
            use super::*;

            #[unsafe(no_mangle)]
            pub extern "C" fn frame_calculate_scale_width(
                frame: *const Frame,
                width: Coordinate,
            ) -> Coordinate {
                if frame.is_null() {
                    return 0.0;
                }
                unsafe { (&*frame).calculate_scale_width(width) }
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn frame_calculate_scale_height(
                frame: *const Frame,
                height: Coordinate,
            ) -> Coordinate {
                if frame.is_null() {
                    return 0.0;
                }
                unsafe { (&*frame).calculate_scale_height(height) }
            }
        }
    }
}
