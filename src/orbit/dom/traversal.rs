use super::node::NodeId;
use super::tree::Dom;

impl Dom {
    pub fn is_descendant(
        &self,
        node: NodeId,
        ancestor: NodeId,
    ) -> bool {
        let mut current = self.parent(node);

        while let Some(parent) = current {
            if parent == ancestor {
                return true;
            }

            current = self.parent(parent);
        }

        false
    }

    pub fn is_inclusive_descendant(
        &self,
        node: NodeId,
        ancestor: NodeId,
    ) -> bool {
        node == ancestor
            || self.is_descendant(node, ancestor)
    }

    pub fn is_ancestor(
        &self,
        ancestor: NodeId,
        node: NodeId,
    ) -> bool {
        self.is_descendant(node, ancestor)
    }

    pub fn is_inclusive_ancestor(
        &self,
        ancestor: NodeId,
        node: NodeId,
    ) -> bool {
        ancestor == node
            || self.is_ancestor(ancestor, node)
    }

    pub fn is_sibling(
        &self,
        a: NodeId,
        b: NodeId,
    ) -> bool {
        if a == b {
            return false;
        }

        match (self.parent(a), self.parent(b)) {
            (Some(a_parent), Some(b_parent)) => {
                a_parent == b_parent
            }
            _ => false,
        }
    }

    pub fn first_child(
        &self,
        node: NodeId,
    ) -> Option<NodeId> {
        self.children(node)
            .first()
            .copied()
    }

    pub fn last_child(
        &self,
        node: NodeId,
    ) -> Option<NodeId> {
        self.children(node)
            .last()
            .copied()
    }

    pub fn index(
        &self,
        node: NodeId,
    ) -> Option<usize> {
        let parent = self.parent(node)?;

        self.children(parent)
            .iter()
            .position(|&child| child == node)
    }

    pub fn previous_sibling(
        &self,
        node: NodeId,
    ) -> Option<NodeId> {
        let parent = self.parent(node)?;
        let index = self.index(node)?;

        if index == 0 {
            return None;
        }

        self.children(parent)
            .get(index - 1)
            .copied()
    }

    pub fn next_sibling(
        &self,
        node: NodeId,
    ) -> Option<NodeId> {
        let parent = self.parent(node)?;
        let index = self.index(node)?;

        self.children(parent)
            .get(index + 1)
            .copied()
    }

    pub fn tree_order(
        &self,
        root: NodeId,
    ) -> Vec<NodeId> {
        let mut result = Vec::new();

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

    pub fn is_preceding(
        &self,
        a: NodeId,
        b: NodeId,
    ) -> bool {
        if self.root(a) != self.root(b) {
            return false;
        }

        let order = self.tree_order(self.root(a));

        let a_index =
            order.iter().position(|&id| id == a);

        let b_index =
            order.iter().position(|&id| id == b);

        match (a_index, b_index) {
            (Some(a), Some(b)) => a < b,
            _ => false,
        }
    }

    pub fn is_following(
        &self,
        a: NodeId,
        b: NodeId,
    ) -> bool {
        self.is_preceding(b, a)
    }
}
