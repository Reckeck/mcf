pub type Coordinate = f32;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Position {
    pub x: Coordinate,
    pub y: Coordinate,
}

#[derive(Default, Debug, Clone, PartialEq)]
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
