use super::node::{
    Node,
    NodeId,
    NodeType,
};

pub struct Dom {
    nodes: Vec<Node>,
}

impl Dom {
    pub fn new() -> Self {
        Self {
            nodes: vec![
                Node::new(NodeType::Document),
            ],
        }
    }

    pub fn create_node(
        &mut self,
        node_type: NodeType,
    ) -> NodeId {
        let id = NodeId(self.nodes.len());

        self.nodes.push(
            Node::new(node_type),
        );

        id
    }

    pub fn parent(
        &self,
        node: NodeId,
    ) -> Option<NodeId> {
        self.nodes[node.0].parent
    }

    pub fn children(
        &self,
        node: NodeId,
    ) -> &[NodeId] {
        &self.nodes[node.0].children
    }

    pub fn root(
        &self,
        mut node: NodeId,
    ) -> NodeId {
        while let Some(parent) =
            self.parent(node)
        {
            node = parent;
        }

        node
    }
}
