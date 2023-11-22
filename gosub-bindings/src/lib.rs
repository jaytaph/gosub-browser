use std::{cell::RefCell, f64::consts, rc::Rc};

use gosub_engine::{
    bytes::{CharIterator, Confidence, Encoding},
    html5::parser::{
        document::{Document, DocumentBuilder},
        Html5Parser,
    },
    render_tree::{self, RenderTree},
};

#[no_mangle]
pub extern "C" fn render_tree_init() -> *mut RenderTree {
    let mut chars = CharIterator::new();
    chars.read_from_str("<html><h1>test</h1></html>", Some(Encoding::UTF8));
    chars.set_confidence(Confidence::Certain);

    let doc = DocumentBuilder::new_document();
    let _ = Html5Parser::parse_document(&mut chars, Document::clone(&doc), None);

    let mut render_tree = Box::new(RenderTree::new(&doc));
    render_tree.build();

    &mut *render_tree
}

#[no_mangle]
pub extern "C" fn render_tree_next_node(render_tree: *const RenderTree) -> usize {
    unsafe {
        // NOTE: I've also tried converting Rc into a raw pointer but that
        // didn't seem to work either...
        let root = (*render_tree).root.borrow();
        let n_children = root.children.len();
        n_children
    }
}
