use super::attribute::AttributeMap;

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
pub struct ElementData {
    pub local_name: String,

    pub namespace: String,

    pub attributes: AttributeMap,
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,

    pub parent: Option<NodeId>,

    pub children: Vec<NodeId>,

    /*
     * Element-specific data.
     *
     * This is Some only for Element nodes.
     */
    pub element: Option<ElementData>,
}

impl Node {
    pub fn new(
        node_type: NodeType,
    ) -> Self {
        Self {
            node_type,

            parent: None,

            children: Vec::new(),

            element: None,
        }
    }

    pub fn new_element(
        local_name: impl Into<String>,
    ) -> Self {
        Self {
            node_type: NodeType::Element,

            parent: None,

            children: Vec::new(),

            element: Some(
                ElementData {
                    local_name:
                        local_name.into(),

                    namespace:
                        "http://www.w3.org/1999/xhtml"
                            .to_string(),

                    attributes:
                        AttributeMap::new(),
                }
            ),
        }
    }

    pub fn is_element(&self) -> bool {
        self.node_type
            == NodeType::Element
    }

    pub fn is_document(&self) -> bool {
        self.node_type
            == NodeType::Document
    }

    pub fn is_text(&self) -> bool {
        self.node_type
            == NodeType::Text
    }

    pub fn is_fragment(&self) -> bool {
        self.node_type
            == NodeType::DocumentFragment
    }
}
