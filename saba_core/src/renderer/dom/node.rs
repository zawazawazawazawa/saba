use crate::renderer::html::attribute::Attribute;
use alloc::format;
use alloc::rc::Rc;
use alloc::rc::Weak;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::fmt::Display;
use core::fmt::Formatter;
use core::str::FromStr;

#[derive(Debug, Clone)]
pub enum NodeKind {
    Document,
    Element(Element),
    Text(String),
}

impl PartialEq for NodeKind {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            NodeKind::Document => matches!(other, NodeKind::Document),
            NodeKind::Element(e1) => match &other {
                NodeKind::Element(e2) => e1.kind == e2.kind,
                _ => false,
            },
            NodeKind::Text(_) => matches!(other, NodeKind::Text(_)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    window: Weak<RefCell<Window>>,
    parent: Weak<RefCell<Node>>,
    first_child: Option<Rc<RefCell<Node>>>,
    last_child: Weak<RefCell<Node>>,
    prev_sibling: Weak<RefCell<Node>>,
    next_sibling: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Node {
            kind,
            window: Weak::new(),
            parent: Weak::new(),
            first_child: None,
            last_child: Weak::new(),
            prev_sibling: Weak::new(),
            next_sibling: None,
        }
    }

    pub fn set_parent(&mut self, parent: Weak<RefCell<Node>>) {
        self.parent = parent;
    }

    pub fn parent(&self) -> Weak<RefCell<Node>> {
        self.parent.clone()
    }

    pub fn set_first_child(&mut self, first_child: Option<Rc<RefCell<Node>>>) {
        self.first_child = first_child;
    }

    pub fn first_child(&self) -> Option<Rc<RefCell<Node>>> {
        self.first_child.clone()
    }

    pub fn set_last_child(&mut self, last_child: Weak<RefCell<Node>>) {
        self.last_child = last_child;
    }

    pub fn last_child(&self) -> Weak<RefCell<Node>> {
        self.last_child.clone()
    }

    pub fn set_prev_sibling(&mut self, prev_sibling: Weak<RefCell<Node>>) {
        self.prev_sibling = prev_sibling;
    }

    pub fn prev_sibling(&self) -> Weak<RefCell<Node>> {
        self.prev_sibling.clone()
    }

    pub fn set_next_sibling(&mut self, next_sibling: Option<Rc<RefCell<Node>>>) {
        self.next_sibling = next_sibling;
    }

    pub fn next_sibling(&self) -> Option<Rc<RefCell<Node>>> {
        self.next_sibling.clone()
    }

    pub fn set_window(&mut self, window: Weak<RefCell<Window>>) {
        self.window = window;
    }

    pub fn kind(&self) -> NodeKind {
        self.kind.clone()
    }

    pub fn get_element(&self) -> Option<Element> {
        match self.kind {
            NodeKind::Document | NodeKind::Text(_) => None,
            NodeKind::Element(ref element) => Some(element.clone()),
        }
    }

    pub fn element_kind(&self) -> Option<ElementKind> {
        match self.kind {
            NodeKind::Document | NodeKind::Text(_) => None,
            NodeKind::Element(ref element) => Some(element.kind()),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    pub document: Rc<RefCell<Node>>,
}

impl Window {
    pub fn new() -> Self {
        let window = Self {
            document: Rc::new(RefCell::new(Node::new(NodeKind::Document))),
        };

        window
            .document
            .borrow_mut()
            .set_window(Rc::downgrade(&Rc::new(RefCell::new(window.clone()))));

        window
    }

    pub fn document(&self) -> Rc<RefCell<Node>> {
        self.document.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    pub kind: ElementKind,
    pub attributes: Vec<Attribute>,
}

impl Element {
    pub fn new(element_name: &str, attributes: Vec<Attribute>) -> Self {
        Self {
            kind: ElementKind::from_str(element_name)
                .expect("failed to convert string to ElementKind"),
            attributes,
        }
    }

    pub fn kind(&self) -> ElementKind {
        self.kind
    }

    pub fn is_block_element(&self) -> bool {
        match self.kind {
            ElementKind::Body | ElementKind::P | ElementKind::H1 | ElementKind::H2 => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Html,
    Head,
    Style,
    Script,
    Body,
    P,
    H1,
    H2,
    A,
}

impl FromStr for ElementKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "html" => Ok(ElementKind::Html),
            "head" => Ok(ElementKind::Head),
            "style" => Ok(ElementKind::Style),
            "script" => Ok(ElementKind::Script),
            "body" => Ok(ElementKind::Body),
            "p" => Ok(ElementKind::P),
            "h1" => Ok(ElementKind::H1),
            "h2" => Ok(ElementKind::H2),
            "a" => Ok(ElementKind::A),
            _ => Err(format!("unimplemented element name: {:?}", s)),
        }
    }
}

impl Display for ElementKind {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let s = match self {
            ElementKind::Html => "html",
            ElementKind::Head => "head",
            ElementKind::Style => "style",
            ElementKind::Script => "script",
            ElementKind::Body => "body",
            ElementKind::H1 => "h1",
            ElementKind::H2 => "h2",
            ElementKind::P => "p",
            ElementKind::A => "a",
        };
        write!(f, "{}", s)
    }
}
