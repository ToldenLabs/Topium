use super::document::DocumentData;
use super::node::{
    Node,
    NodeId,
    NodeType,
};

/// The DOM tree.
///
/// NodeId values are indexes into `nodes`.
pub struct Dom {
    pub(crate) nodes: Vec<Node>,

    pub(crate) document_data: DocumentData,
}

impl Dom {
    /// Creates a new DOM with a Document node at NodeId(0).
    pub fn new() -> Self {
        let mut dom = Self {
            nodes: Vec::new(),

            document_data:
                DocumentData::new(),
        };

        dom.nodes.push(
            Node::new(
                NodeType::Document
            )
        );

        dom
    }

    /// Returns a node by ID.
    pub fn get(
        &self,
        id: NodeId,
    ) -> Option<&Node> {
        self.nodes.get(id.0)
    }

    /// Returns a mutable node by ID.
    pub fn get_mut(
        &mut self,
        id: NodeId,
    ) -> Option<&mut Node> {
        self.nodes.get_mut(id.0)
    }

    /// Returns the parent of a node.
    pub fn parent(
        &self,
        id: NodeId,
    ) -> Option<NodeId> {
        self.get(id)?.parent
    }

    /// Returns the children of a node.
    pub fn children(
        &self,
        id: NodeId,
    ) -> &[NodeId] {
        match self.get(id) {
            Some(node) => &node.children,
            None => &[],
        }
    }

    /// Creates a generic node.
    pub(crate) fn create_node(
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

    /// Appends a child to a parent.
    ///
    /// Tree structure is updated here.
    /// DOM insertion hooks will be added later.
    pub fn append_child(
        &mut self,
        parent: NodeId,
        child: NodeId,
    ) -> Result<(), TreeError> {
        if self.get(parent).is_none() {
            return Err(
                TreeError::InvalidParent
            );
        }

        if self.get(child).is_none() {
            return Err(
                TreeError::InvalidChild
            );
        }

        /*
         * A node cannot be its own parent.
         */
        if parent == child {
            return Err(
                TreeError::HierarchyRequest
            );
        }

        /*
         * Prevent creation of cycles.
         *
         * parent cannot be inside child.
         */
        if self.is_inclusive_descendant(
            parent,
            child,
        ) {
            return Err(
                TreeError::HierarchyRequest
            );
        }

        /*
         * If the child already has a parent,
         * detach it first.
         */
        if self.parent(child).is_some() {
            self.remove_child(child)?;
        }

        /*
         * Add child to parent's children.
         */
        {
            let parent_node =
                self.get_mut(parent)
                    .unwrap();

            parent_node
                .children
                .push(child);
        }

        /*
         * Set child's parent.
         */
        {
            let child_node =
                self.get_mut(child)
                    .unwrap();

            child_node.parent =
                Some(parent);
        }

        Ok(())
    }

    /// Removes a node from its parent.
    pub fn remove_child(
        &mut self,
        child: NodeId,
    ) -> Result<(), TreeError> {
        let parent =
            self.parent(child)
                .ok_or(
                    TreeError::NotChild
                )?;

        {
            let parent_node =
                self.get_mut(parent)
                    .unwrap();

            let position =
                parent_node
                    .children
                    .iter()
                    .position(
                        |&id| id == child
                    )
                    .ok_or(
                        TreeError::NotChild
                    )?;

            parent_node
                .children
                .remove(position);
        }

        {
            let child_node =
                self.get_mut(child)
                    .unwrap();

            child_node.parent = None;
        }

        Ok(())
    }

    /// Returns the root of a node.
    pub fn root(
        &self,
        id: NodeId,
    ) -> NodeId {
        let mut current = id;

        while let Some(parent) =
            self.parent(current)
        {
            current = parent;
        }

        current
    }

    /// Returns whether `node` is a descendant
    /// of `ancestor`.
    pub fn is_descendant(
        &self,
        node: NodeId,
        ancestor: NodeId,
    ) -> bool {
        let mut current =
            self.parent(node);

        while let Some(parent) =
            current
        {
            if parent == ancestor {
                return true;
            }

            current =
                self.parent(parent);
        }

        false
    }

    /// Returns whether node is an inclusive
    /// descendant of ancestor.
    pub fn is_inclusive_descendant(
        &self,
        node: NodeId,
        ancestor: NodeId,
    ) -> bool {
        node == ancestor
            || self.is_descendant(
                node,
                ancestor,
            )
    }

    /// Returns whether two nodes are siblings.
    pub fn is_sibling(
        &self,
        a: NodeId,
        b: NodeId,
    ) -> bool {
        if a == b {
            return false;
        }

        match (
            self.parent(a),
            self.parent(b),
        ) {
            (
                Some(parent_a),
                Some(parent_b),
            ) => parent_a == parent_b,

            _ => false,
        }
    }

    /// Returns the first child.
    pub fn first_child(
        &self,
        id: NodeId,
    ) -> Option<NodeId> {
        self.children(id)
            .first()
            .copied()
    }

    /// Returns the last child.
    pub fn last_child(
        &self,
        id: NodeId,
    ) -> Option<NodeId> {
        self.children(id)
            .last()
            .copied()
    }

    /// Returns the previous sibling.
    pub fn previous_sibling(
        &self,
        id: NodeId,
    ) -> Option<NodeId> {
        let parent =
            self.parent(id)?;

        let children =
            self.children(parent);

        let index =
            children
                .iter()
                .position(
                    |&child| child == id
                )?;

        if index == 0 {
            return None;
        }

        children
            .get(index - 1)
            .copied()
    }

    /// Returns the next sibling.
    pub fn next_sibling(
        &self,
        id: NodeId,
    ) -> Option<NodeId> {
        let parent =
            self.parent(id)?;

        let children =
            self.children(parent);

        let index =
            children
                .iter()
                .position(
                    |&child| child == id
                )?;

        children
            .get(index + 1)
            .copied()
    }

    /// Returns the index of a node amongst
    /// its siblings.
    pub fn index(
        &self,
        id: NodeId,
    ) -> Option<usize> {
        let parent =
            self.parent(id)?;

        self.children(parent)
            .iter()
            .position(
                |&child| child == id
            )
    }

    /// Pre-order depth-first tree traversal.
    pub fn tree_order(
        &self,
        root: NodeId,
    ) -> Vec<NodeId> {
        let mut result =
            Vec::new();

        self.collect_tree_order(
            root,
            &mut result,
        );

        result
    }

    fn collect_tree_order(
        &self,
        node: NodeId,
        result: &mut Vec<NodeId>,
    ) {
        result.push(node);

        for child in self.children(node) {
            self.collect_tree_order(
                *child,
                result,
            );
        }
    }
}

impl Default for Dom {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeError {
    InvalidParent,
    InvalidChild,
    HierarchyRequest,
    NotChild,
}
