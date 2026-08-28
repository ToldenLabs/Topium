#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct NodeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Document,
    Element,
    Text,
    Comment,
    DocumentType,
    DocumentFragment,
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
}

impl Node {
    pub fn new(node_type: NodeType) -> Self {
        Self {
            node_type,
            parent: None,
            children: Vec::new(),
        }
    }
}
