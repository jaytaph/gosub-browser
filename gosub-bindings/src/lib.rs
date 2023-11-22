use std::{cell::RefCell, f64::consts, ptr, rc::Rc};

use gosub_engine::{
    bytes::{CharIterator, Confidence, Encoding},
    html5::parser::{
        document::{Document, DocumentBuilder},
        Html5Parser,
    },
    render_tree::{self, Node, RenderTree},
};

#[no_mangle]
/// Initialize a render tree and return a pointer to the root node of the tree.
// This may need to change, though, because we can't free unless we keep this pointer
pub extern "C" fn render_tree_init() -> *const RefCell<Node> {
    let mut chars = CharIterator::new();
    chars.read_from_str("<html><h1>test</h1></html>", Some(Encoding::UTF8));
    chars.set_confidence(Confidence::Certain);

    let doc = DocumentBuilder::new_document();
    let _ = Html5Parser::parse_document(&mut chars, Document::clone(&doc), None);

    let mut render_tree = RenderTree::new(&doc);
    render_tree.build();

    Rc::into_raw(render_tree.root)
}

#[no_mangle]
// This is a test function for now to make sure pointers are working, but will
// return the next node in the render tree
pub extern "C" fn render_tree_next_node(node: *const RefCell<Node>) -> usize {
    unsafe {
        let n_children = (*node).borrow().children.len();
        n_children
    }
}

// TODO: add a render_tree_free() to cleanup memory
