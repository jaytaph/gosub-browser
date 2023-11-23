use std::{ffi::CString, os::raw::c_char};

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct TextNode {
    /// Body of the text node that will be drawn
    pub value: *mut c_char,
    pub font: *mut c_char,
    pub font_size: f32,
    pub is_bold: bool,
    // TODO: color, styles, visiblity, etc
}

impl TextNode {
    fn new(fs: f32, bold: bool) -> Self {
        Self {
            value: CString::new("").expect("").into_raw(),
            font: CString::new("Times New Roman").expect("").into_raw(),
            font_size: fs,
            is_bold: bold,
        }
    }

    pub fn new_heading1() -> Self {
        TextNode::new(37., true)
    }

    pub fn new_heading2() -> Self {
        TextNode::new(27.5, true)
    }

    pub fn new_heading3() -> Self {
        TextNode::new(21.5, true)
    }

    pub fn new_heading4() -> Self {
        TextNode::new(18.5, true)
    }

    pub fn new_heading5() -> Self {
        TextNode::new(15.5, true)
    }

    pub fn new_heading6() -> Self {
        TextNode::new(12., true)
    }

    pub fn new_paragraph() -> Self {
        TextNode::new(18.5, false)
    }
}
