use super::node::{
    Node,
    NodeId,
    NodeType,
};
use super::quirks::QuirksMode;
use super::tree::Dom;

/// Data belonging to a Document node.
#[derive(Debug)]
pub struct DocumentData {
    /// The document's current quirks mode.
    pub quirks_mode: QuirksMode,
}

impl Default for DocumentData {
    fn default() -> Self {
        Self {
            quirks_mode:
                QuirksMode::NoQuirks,
        }
    }
}

impl DocumentData {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Document-related DOM operations.
impl Dom {
    /// The document node is the root node.
    pub fn document() -> NodeId {
        NodeId(0)
    }

    /// Creates a new generic node.
    pub fn create_node(
        &mut self,
        node_type: NodeType,
    ) -> NodeId {
        let id =
            NodeId(self.nodes.len());

        self.nodes.push(
            Node::new(node_type)
        );

        id
    }

    /// Creates an HTML element.
    pub fn create_element(
        &mut self,
        local_name: impl Into<String>,
    ) -> NodeId {
        let id =
            NodeId(self.nodes.len());

        self.nodes.push(
            Node::new_element(
                local_name
            )
        );

        id
    }

    /// Creates a text node.
    pub fn create_text_node(
        &mut self,
    ) -> NodeId {
        self.create_node(
            NodeType::Text
        )
    }

    /// Creates a comment node.
    pub fn create_comment(
        &mut self,
    ) -> NodeId {
        self.create_node(
            NodeType::Comment
        )
    }

    /// Creates a document fragment.
    pub fn create_document_fragment(
        &mut self,
    ) -> NodeId {
        self.create_node(
            NodeType::DocumentFragment
        )
    }

    /// Returns the document's element.
    pub fn document_element(
        &self,
    ) -> Option<NodeId> {
        self.children(Self::document())
            .iter()
            .copied()
            .find(|node| {
                self.nodes[node.0]
                    .node_type
                    == NodeType::Element
            })
    }

    /// Returns the document's quirks mode.
    pub fn quirks_mode(
        &self,
    ) -> QuirksMode {
        self.document_data
            .quirks_mode
    }

    /// Changes the document's quirks mode.
    ///
    /// The HTML parser will eventually be responsible
    /// for determining this value from the DOCTYPE.
    pub fn set_quirks_mode(
        &mut self,
        mode: QuirksMode,
    ) {
        self.document_data
            .quirks_mode = mode;
    }

    /// Returns true when the document is
    /// in full quirks mode.
    pub fn is_quirks_mode(
        &self,
    ) -> bool {
        self.quirks_mode()
            .is_quirks()
    }

    /// Returns true when the document is
    /// in limited quirks mode.
    pub fn is_limited_quirks_mode(
        &self,
    ) -> bool {
        self.quirks_mode()
            .is_limited_quirks()
    }

    /// Returns true when the document is
    /// in standards mode.
    pub fn is_no_quirks_mode(
        &self,
    ) -> bool {
        self.quirks_mode()
            .is_no_quirks()
    }
}
