use super::node::{
    NodeId,
    NodeType,
};
use super::tree::Dom;

impl Dom {
    pub fn document() -> NodeId {
        NodeId(0)
    }

    pub fn create_element(
        &mut self,
        name: impl Into<String>,
    ) -> NodeId {
        /*
         * Element-specific metadata will eventually
         * be stored alongside the Node itself.
         */
        let _name = name.into();

        self.create_node(
            NodeType::Element
        )
    }

    pub fn create_text_node(
        &mut self,
    ) -> NodeId {
        self.create_node(
            NodeType::Text
        )
    }

    pub fn create_comment(
        &mut self,
    ) -> NodeId {
        self.create_node(
            NodeType::Comment
        )
    }

    pub fn create_document_fragment(
        &mut self,
    ) -> NodeId {
        self.create_node(
            NodeType::DocumentFragment
        )
    }

    pub fn document_element(
        &self,
    ) -> Option<NodeId> {
        self.children(Self::document())
            .iter()
            .copied()
            .find(|node| {
                self.nodes[node.0].node_type
                    == NodeType::Element
            })
    }
}
