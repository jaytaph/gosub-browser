use std::{cell::RefCell, f64::consts, ptr, rc::Rc};

use gosub_engine::{
    bytes::{CharIterator, Confidence, Encoding},
    html5::parser::{
        document::{Document, DocumentBuilder},
        Html5Parser,
    },
    render_tree::{Node, NodeType, RenderTree, TreeIterator},
};

#[no_mangle]
/// Initialize a render tree and return an owning pointer to it.
/// If the HTML fails to parse, returns a NULL pointer.
pub extern "C" fn render_tree_init() -> *mut RenderTree {
    let mut chars = CharIterator::new();
    chars.read_from_str("<html><h1>test</h1></html>", Some(Encoding::UTF8));
    chars.set_confidence(Confidence::Certain);

    let doc = DocumentBuilder::new_document();
    let parse_result = Html5Parser::parse_document(&mut chars, Document::clone(&doc), None);

    if let Ok(_) = parse_result {
        let mut render_tree = Box::new(RenderTree::new(&doc));
        render_tree.build();

        Box::into_raw(render_tree)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// Construct a tree iterator for a render tree and return an owning pointer to it.
pub extern "C" fn render_tree_iterator_init(render_tree: *const RenderTree) -> *mut TreeIterator {
    unsafe {
        let tree_iterator = Box::new(TreeIterator::new(&(*render_tree)));
        Box::into_raw(tree_iterator)
    }
}

#[no_mangle]
/// Takes a tree_iterator and returns a non-owning pointer to the next node
pub extern "C" fn render_tree_next_node(tree_iterator: *mut TreeIterator) -> *const Node {
    unsafe {
        let next = (*tree_iterator).next();
        if next.is_none() {
            return ptr::null();
        }
        next.unwrap().as_ptr() as *const Node
    }
}

#[no_mangle]
pub extern "C" fn render_tree_get_node_data(node: *const Node, node_data: *mut NodeType) {
    unsafe {
        ptr::copy_nonoverlapping(
            &(*node).node_type,
            node_data,
            std::mem::size_of::<NodeType>(),
        );
    }
}

// TODO: add a render_tree_free() to cleanup memory
