use super::node::{
    NodeId,
    NodeType,
};
use super::quirks::QuirksMode;
use super::tree::Dom;

/// State associated with a Document.
#[derive(Debug)]
pub struct DocumentData {
    /// The document's current quirks mode.
    pub quirks_mode: QuirksMode,
}

impl DocumentData {
    pub fn new() -> Self {
        Self {
            quirks_mode:
                QuirksMode::NoQuirks,
        }
    }
}

impl Default for DocumentData {
    fn default() -> Self {
        Self::new()
    }
}

impl Dom {
    /// NodeId(0) is always the Document node.
    pub const DOCUMENT_NODE: NodeId =
        NodeId(0);

    /// Returns the Document node.
    pub fn document() -> NodeId {
        Self::DOCUMENT_NODE
    }

    /// Creates an HTML Element.
    ///
    /// Element-specific data is stored directly
    /// inside the Node.
    pub fn create_element(
        &mut self,
        local_name: impl Into<String>,
    ) -> NodeId {
        let id =
            NodeId(self.nodes.len());

        self.nodes.push(
            super::node::Node::new_element(
                local_name
            )
        );

        id
    }

    /// Creates a Text node.
    pub fn create_text_node(
        &mut self,
    ) -> NodeId {
        self.create_node(
            NodeType::Text
        )
    }

    /// Creates a Comment node.
    pub fn create_comment(
        &mut self,
    ) -> NodeId {
        self.create_node(
            NodeType::Comment
        )
    }

    /// Creates a DocumentFragment.
    pub fn create_document_fragment(
        &mut self,
    ) -> NodeId {
        self.create_node(
            NodeType::DocumentFragment
        )
    }

    /// Returns the document element.
    ///
    /// This is the first Element child of the
    /// Document node.
    pub fn document_element(
        &self,
    ) -> Option<NodeId> {
        self.children(
            Self::DOCUMENT_NODE
        )
        .iter()
        .copied()
        .find(|&node| {
            self.get(node)
                .map(|node| {
                    node.node_type
                        == NodeType::Element
                })
                .unwrap_or(false)
        })
    }

    /// Returns the document's quirks mode.
    pub fn quirks_mode(
        &self,
    ) -> QuirksMode {
        self.document_data
            .quirks_mode
    }

    /// Sets the document's quirks mode.
    ///
    /// The HTML parser will eventually determine
    /// this from the document's DOCTYPE.
    pub fn set_quirks_mode(
        &mut self,
        mode: QuirksMode,
    ) {
        self.document_data
            .quirks_mode = mode;
    }

    /// True when the document is in full quirks mode.
    pub fn is_quirks_mode(
        &self,
    ) -> bool {
        self.quirks_mode()
            == QuirksMode::Quirks
    }

    /// True when the document is in limited quirks mode.
    pub fn is_limited_quirks_mode(
        &self,
    ) -> bool {
        self.quirks_mode()
            == QuirksMode::LimitedQuirks
    }

    /// True when the document is in standards mode.
    pub fn is_no_quirks_mode(
        &self,
    ) -> bool {
        self.quirks_mode()
            == QuirksMode::NoQuirks
    }
}
