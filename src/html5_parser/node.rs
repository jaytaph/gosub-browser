use crate::html5_parser::node::data::comment::CommentData;
use crate::html5_parser::node::data::document::DocumentData;
use crate::html5_parser::node::data::element::ElementData;
use crate::html5_parser::node::data::text::TextData;
use derive_more::Display;
use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;


pub const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
pub const MATHML_NAMESPACE: &str = "http://www.w3.org/1998/Math/MathML";
pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";
pub const XLINK_NAMESPACE: &str = "http://www.w3.org/1999/xlink";
pub const XML_NAMESPACE: &str = "http://www.w3.org/XML/1998/namespace";
pub const XMLNS_NAMESPACE: &str = "http://www.w3.org/2000/xmlns/";

pub mod arena;
pub mod data;

/// Different types of nodes
#[derive(Debug, PartialEq)]
pub enum NodeType {
    Document,
    Text,
    Comment,
    Element,
}

/// Different type of node data
#[derive(Debug, Clone, PartialEq)]
pub enum NodeData {
    Document(DocumentData),
    Text(TextData),
    Comment(CommentData),
    Element(ElementData),
}

/// Id used to identify a node
#[derive(Copy, Debug, Default, Eq, Hash, PartialEq, Display)]
pub struct NodeId(pub usize);

impl From<NodeId> for usize {
    fn from(value: NodeId) -> Self {
        value.0
    }
}

impl From<usize> for NodeId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Default for &NodeId {
    fn default() -> Self {
        &NodeId(0)
    }
}

impl NodeId {
    // TODO: Drop Default derive and only use 0 for the root, or choose another id for the root
    pub const ROOT_NODE: usize = 0;

    pub fn root() -> Self {
        Self(Self::ROOT_NODE)
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }

    pub fn is_root(&self) -> bool {
        self.0 == Self::ROOT_NODE
    }

    pub fn next(&self) -> Self {
        // Might panic
        Self(self.0 + 1)
    }

    pub fn prev(&self) -> Self {
        // Might panic
        Self(self.0 - 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node(RefCell<Inner>);

impl Node {
    /// Create a new document node
    pub fn new_document() -> Self {
        Self(RefCell::new(Inner {
            id: Default::default(),
            named_id: None,
            parent: None,
            children: vec![],
            data: NodeData::Document(DocumentData::new()),
            name: "".to_string(),
            namespace: None,
        }))
    }

    /// Create a new element node with the given name and attributes and namespace
    pub fn new_element(name: &str, attributes: HashMap<String, String>, namespace: &str) -> Self {
        Self(RefCell::new(Inner {
            id: Default::default(),
            named_id: None,
            parent: None,
            children: vec![],
            data: NodeData::Element(ElementData::with_name_and_attributes(name, attributes)),
            name: name.to_string(),
            namespace: Some(namespace.into()),
        }))
    }

    /// Create a new comment node
    pub fn new_comment(value: &str) -> Self {
        Self(RefCell::new(Inner {
            id: Default::default(),
            named_id: None,
            parent: None,
            children: vec![],
            data: NodeData::Comment(CommentData::with_value(value)),
            name: "".to_string(),
            namespace: None,
        }))
    }

    /// Create a new text node
    pub fn new_text(value: &str) -> Self {
        Self(RefCell::new(Inner {
            id: Default::default(),
            named_id: None,
            parent: None,
            children: vec![],
            data: NodeData::Text(TextData::with_value(value)),
            name: "".to_string(),
            namespace: None,
        }))
    }

    pub fn get_named_id(&self) -> Option<String> {
        self.0.borrow().get_named_id()
    }

    pub fn set_named_id(&mut self, named_id: &str) {
        self.0.get_mut().set_named_id(named_id)
    }

    pub fn has_named_id(&self) -> bool {
        self.0.borrow().has_named_id()
    }

    pub fn is_special(&self) -> bool {
        self.0.borrow().is_special()
    }

    pub fn data(&self) -> Ref<'_, NodeData> {
        Ref::map(self.0.borrow(), |inner| &inner.data)
    }

    pub fn with_data<T>(&mut self, f: impl FnOnce(&mut NodeData) -> T) -> T {
        let inner = self.0.get_mut();
        f(&mut inner.data)
    }

    pub fn id(&self) -> NodeId {
        self.0.borrow().id
    }

    pub fn set_id(&mut self, node_id: NodeId) {
        self.0.get_mut().id = node_id
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.0.borrow().parent
    }

    pub fn set_parent(&mut self, parent_id: Option<NodeId>) {
        self.0.get_mut().parent = parent_id
    }

    pub fn children(&self) -> Ref<'_, Vec<NodeId>> {
        Ref::map(self.0.borrow(), |inner| &inner.children)
    }

    pub fn children_mut(&mut self) -> RefMut<'_, Vec<NodeId>> {
        RefMut::map(self.0.borrow_mut(), |inner| &mut inner.children)
    }

    pub fn push_child(&mut self, child_id: NodeId) {
        self.0.get_mut().children.push(child_id)
    }

    pub fn name(&self) -> Ref<'_, String> {
        Ref::map(self.0.borrow(), |inner| &inner.name)
    }

    pub fn namespace(&self) -> Ref<'_, Option<String>> {
        Ref::map(self.0.borrow(), |inner| &inner.namespace)
    }

    pub fn push(&mut self, node_id: NodeId) {
        self.0.get_mut().children.push(node_id)
    }

    pub fn matches_tag_and_attrs(&self, other: &Self) -> bool {
        self.0.borrow().matches_tag_and_attrs(&other.0.borrow())
    }
}

impl NodeTrait for Node {
    fn type_of(&self) -> NodeType {
        self.0.borrow().type_of()
    }
}

/// Node that resembles a DOM node
#[derive(Debug, PartialEq)]
pub struct Inner {
    /// ID of the node, 0 is always the root / document node
    pub id: NodeId,
    /// Named ID of the node, from the "id" attribute on an HTML element
    pub named_id: Option<String>,
    /// parent of the node, if any
    pub parent: Option<NodeId>,
    /// children of the node
    pub children: Vec<NodeId>,
    /// name of the node, or empty when it's not a tag
    pub name: String,
    /// namespace of the node
    pub namespace: Option<String>,
    /// actual data of the node
    pub data: NodeData,
}

impl Inner {
    /// This will only compare against the tag, namespace and attributes. Both nodes could still have
    /// other parents and children.
    pub fn matches_tag_and_attrs(&self, other: &Self) -> bool {
        self.name == other.name && self.namespace == other.namespace && self.data == other.data
    }
}

impl Clone for Inner {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            named_id: self.named_id.clone(),
            parent: self.parent,
            children: self.children.clone(),
            name: self.name.clone(),
            namespace: self.namespace.clone(),
            data: self.data.clone(),
        }
    }
}

impl Inner {
    /// Returns true if the given node is a "formatting" node
    pub fn is_formatting(&self) -> bool {
        self.namespace == Some(HTML_NAMESPACE.into())
            && FORMATTING_HTML_ELEMENTS.contains(&self.name.as_str())
    }

    /// Returns true if the given node is "special" node based on the namespace and name
    pub fn is_special(&self) -> bool {
        if self.namespace == Some(HTML_NAMESPACE.into())
            && SPECIAL_HTML_ELEMENTS.contains(&self.name.as_str())
        {
            return true;
        }
        if self.namespace == Some(MATHML_NAMESPACE.into())
            && SPECIAL_MATHML_ELEMENTS.contains(&self.name.as_str())
        {
            return true;
        }
        if self.namespace == Some(SVG_NAMESPACE.into())
            && SPECIAL_SVG_ELEMENTS.contains(&self.name.as_str())
        {
            return true;
        }

        false
    }

    /// Check if node has a named ID
    pub fn has_named_id(&self) -> bool {
        if self.type_of() != NodeType::Element {
            return false;
        }

        self.named_id.is_some()
    }

    /// Set named ID (only applies to Element type, does nothing otherwise)
    pub fn set_named_id(&mut self, named_id: &str) {
        if self.type_of() == NodeType::Element {
            self.named_id = Some(named_id.to_owned());
            let data = self.data.borrow_mut();
            if let NodeData::Element(element) = data {
                element.attributes.insert("id", named_id);
            }
        }
    }

    /// Get named ID. If not present or type is not Element, returns None
    pub fn get_named_id(&self) -> Option<String> {
        if self.type_of() != NodeType::Element {
            return None;
        }

        if !self.has_named_id() {
            return None;
        }

        // don't want to return the actual internal String
        self.named_id.clone()
    }
}

pub trait NodeTrait {
    /// Return the token type of the given token
    fn type_of(&self) -> NodeType;
}

// Each node implements the NodeTrait and has a type_of that will return the node type.
impl NodeTrait for Inner {
    fn type_of(&self) -> NodeType {
        match &self.data {
            NodeData::Document { .. } => NodeType::Document,
            NodeData::Text { .. } => NodeType::Text,
            NodeData::Comment { .. } => NodeType::Comment,
            NodeData::Element { .. } => NodeType::Element,
        }
    }
}

/// The HTML elements that are considered formatting elements
pub static FORMATTING_HTML_ELEMENTS: [&str; 14] = [
    "a", "b", "big", "code", "em", "font", "i", "nobr", "s", "small", "strike", "strong", "tt", "u",
];

/// The HTML elements that are considered special elements
pub static SPECIAL_HTML_ELEMENTS: [&str; 83] = [
    "address",
    "applet",
    "area",
    "article",
    "aside",
    "base",
    "basefont",
    "bgsound",
    "blockquote",
    "body",
    "br",
    "button",
    "caption",
    "center",
    "col",
    "colgroup",
    "dd",
    "details",
    "dir",
    "div",
    "dl",
    "dt",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "frame",
    "frameset",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "iframe",
    "img",
    "input",
    "keygen",
    "li",
    "link",
    "listing",
    "main",
    "marquee",
    "menu",
    "meta",
    "nav",
    "noembed",
    "noframes",
    "noscript",
    "object",
    "ol",
    "p",
    "param",
    "plaintext",
    "pre",
    "script",
    "search",
    "section",
    "select",
    "source",
    "style",
    "summary",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "title",
    "tr",
    "track",
    "ul",
    "wbr",
    "xmp",
];

/// The MathML elements that are considered special elements
pub static SPECIAL_MATHML_ELEMENTS: [&str; 6] = ["mi", "mo", "mn", "ms", "mtext", "annotation-xml"];

/// The SVG elements that are considered special elements
pub static SPECIAL_SVG_ELEMENTS: [&str; 3] = ["foreignObject", "desc", "title"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_document() {
        let node = Node::new_document();
        assert_eq!(node.id(), NodeId::default());
        assert_eq!(node.parent(), None);
        assert!(node.children().is_empty());
        assert_eq!(*node.name(), "".to_string());
        assert_eq!(*node.namespace(), None);

        let data = &*node.data();
        match data {
            NodeData::Document(_) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn new_element() {
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "test".to_string());
        let node = Node::new_element("div", attributes.clone(), HTML_NAMESPACE);
        assert_eq!(node.id(), NodeId::default());
        assert_eq!(node.parent(), None);
        assert!(node.children().is_empty());
        assert_eq!(*node.name(), "div".to_string());
        assert_eq!(*node.namespace(), Some(HTML_NAMESPACE.into()));
        let NodeData::Element(ElementData {
            name, attributes, ..
        }) = &*node.data()
        else {
            panic!()
        };
        assert_eq!(name, "div");
        assert!(attributes.contains("id"));
        assert_eq!(attributes.get("id").unwrap(), "test");
    }

    #[test]
    fn new_comment() {
        let node = Node::new_comment("test");
        assert_eq!(node.id(), NodeId::default());
        assert_eq!(node.parent(), None);
        assert!(node.children().is_empty());
        assert_eq!(*node.name(), "".to_string());
        assert_eq!(*node.namespace(), None);
        let NodeData::Comment(CommentData { value, .. }) = &*node.data() else {
            panic!()
        };
        assert_eq!(value, "test");
    }

    #[test]
    fn new_text() {
        let node = Node::new_text("test");
        assert_eq!(node.id(), NodeId::default());
        assert_eq!(node.parent(), None);
        assert!(node.children().is_empty());
        assert_eq!(*node.name(), "".to_string());
        assert_eq!(*node.namespace(), None);
        let NodeData::Text(TextData { value }) = &*node.data() else {
            panic!()
        };
        assert_eq!(value, "test");
    }

    #[test]
    fn is_special() {
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "test".to_string());
        let node = Node::new_element("div", attributes, HTML_NAMESPACE);
        assert!(node.is_special());
    }

    #[test]
    fn type_of() {
        let node = Node::new_document();
        assert_eq!(node.type_of(), NodeType::Document);
        let node = Node::new_text("test");
        assert_eq!(node.type_of(), NodeType::Text);
        let node = Node::new_comment("test");
        assert_eq!(node.type_of(), NodeType::Comment);
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "test".to_string());
        let node = Node::new_element("div", attributes, HTML_NAMESPACE);
        assert_eq!(node.type_of(), NodeType::Element);
    }

    #[test]
    fn special_html_elements() {
        for element in SPECIAL_HTML_ELEMENTS.iter() {
            let mut attributes = HashMap::new();
            attributes.insert("id".to_string(), "test".to_string());
            let node = Node::new_element(element, attributes, HTML_NAMESPACE);
            assert!(node.is_special());
        }
    }

    #[test]
    fn special_mathml_elements() {
        for element in SPECIAL_MATHML_ELEMENTS.iter() {
            let mut attributes = HashMap::new();
            attributes.insert("id".to_string(), "test".to_string());
            let node = Node::new_element(element, attributes, MATHML_NAMESPACE);
            assert!(node.is_special());
        }
    }

    #[test]
    fn special_svg_elements() {
        for element in SPECIAL_SVG_ELEMENTS.iter() {
            let mut attributes = HashMap::new();
            attributes.insert("id".to_string(), "test".to_string());
            let node = Node::new_element(element, attributes, SVG_NAMESPACE);
            assert!(node.is_special());
        }
    }

    #[test]
    fn type_of_node() {
        let node = Node::new_document();
        assert_eq!(node.type_of(), NodeType::Document);
        let node = Node::new_text("test");
        assert_eq!(node.type_of(), NodeType::Text);
        let node = Node::new_comment("test");
        assert_eq!(node.type_of(), NodeType::Comment);
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "test".to_string());
        let node = Node::new_element("div", attributes, HTML_NAMESPACE);
        assert_eq!(node.type_of(), NodeType::Element);
    }

    #[test]
    fn contains_attribute() {
        let mut attr = HashMap::new();
        attr.insert("x".to_string(), "value".to_string());
        let node = Node::new_element("node", attr.clone(), HTML_NAMESPACE);
        let NodeData::Element(ElementData { attributes, .. }) = &*node.data() else {
            panic!()
        };
        assert!(attributes.contains("x"));
        assert!(!attributes.contains("z"));
    }

    #[test]
    fn insert_attribute() {
        let attr = HashMap::new();
        let mut node = Node::new_element("name", attr.clone(), HTML_NAMESPACE);
        node.with_data(|data| {
            let NodeData::Element(element) = data else {
                panic!()
            };
            element.attributes.insert("key", "value");
            assert_eq!(element.attributes.get("key").unwrap(), "value");
        });
    }

    #[test]
    fn remove_attribute() {
        let mut attr = HashMap::new();
        attr.insert("key".to_string(), "value".to_string());
        let mut node = Node::new_element("name", attr.clone(), HTML_NAMESPACE);
        node.with_data(|data| {
            let NodeData::Element(ElementData { attributes, .. }) = data else {
                panic!()
            };
            attributes.remove("key");
            assert!(!attributes.contains("key"));
        });
    }

    #[test]
    fn get_attribute() {
        let mut attr = HashMap::new();
        attr.insert("key".to_string(), "value".to_string());
        let node = Node::new_element("name", attr.clone(), HTML_NAMESPACE);
        let NodeData::Element(ElementData { attributes, .. }) = &*node.data() else {
            panic!()
        };
        assert_eq!(attributes.get("key").unwrap(), "value");
    }

    #[test]
    fn get_mut_attribute() {
        let mut attr = HashMap::new();
        attr.insert("key".to_string(), "value".to_string());
        let mut node = Node::new_element("name", attr.clone(), HTML_NAMESPACE);
        node.with_data(|data| {
            let NodeData::Element(ElementData { attributes, .. }) = data else {
                panic!()
            };
            let attr_val = attributes.get_mut("key").unwrap();
            attr_val.push_str(" appended");
            assert_eq!(attributes.get("key").unwrap(), "value appended");
        });
    }

    #[test]
    fn clear_attributes() {
        let mut attr = HashMap::new();
        attr.insert("key".to_string(), "value".to_string());
        let mut node = Node::new_element("name", attr.clone(), HTML_NAMESPACE);
        node.with_data(|data| {
            let NodeData::Element(ElementData { attributes, .. }) = data else {
                panic!()
            };
            attributes.clear();
            assert!(attributes.is_empty());
        });
    }

    #[test]
    fn has_attributes() {
        let attr = HashMap::new();
        let mut node = Node::new_element("name", attr.clone(), HTML_NAMESPACE);
        node.with_data(|data| {
            let NodeData::Element(ElementData { attributes, .. }) = data else {
                panic!()
            };
            assert!(attributes.is_empty());
            attributes.insert("key", "value");
            assert!(!attributes.is_empty());
        });
    }
}
