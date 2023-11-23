/// Rectangular dimensions commonly used for certain properties such as margin/padding
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Rectangle {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            top: 0.,
            left: 0.,
            right: 0.,
            bottom: 0.,
        }
    }
}

impl Rectangle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_values(top: f32, left: f32, right: f32, bottom: f32) -> Self {
        Self {
            top,
            left,
            right,
            bottom,
        }
    }
}
