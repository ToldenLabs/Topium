use super::attribute::Attribute;
use super::node::{
    Node,
    NodeId,
    NodeType,
};
use super::tree::Dom;

impl Dom {
    pub fn element(
        &self,
        node: NodeId,
    ) -> Option<&Node> {
        let node_ref =
            &self.nodes[node.0];

        if node_ref.node_type
            != NodeType::Element
        {
            return None;
        }

        Some(node_ref)
    }

    pub fn element_mut(
        &mut self,
        node: NodeId,
    ) -> Option<&mut Node> {
        let node_ref =
            &mut self.nodes[node.0];

        if node_ref.node_type
            != NodeType::Element
        {
            return None;
        }

        Some(node_ref)
    }

    pub fn local_name(
        &self,
        node: NodeId,
    ) -> Option<&str> {
        self.element(node)?
            .element
            .as_ref()?
            .local_name
            .as_str()
            .into()
    }

    pub fn namespace(
        &self,
        node: NodeId,
    ) -> Option<&str> {
        self.element(node)?
            .element
            .as_ref()?
            .namespace
            .as_str()
            .into()
    }

    pub fn get_attribute(
        &self,
        node: NodeId,
        name: &str,
    ) -> Option<&str> {
        self.element(node)?
            .element
            .as_ref()?
            .attributes
            .get(name)
            .map(|attribute| {
                attribute.value.as_str()
            })
    }

    pub fn has_attribute(
        &self,
        node: NodeId,
        name: &str,
    ) -> bool {
        self.get_attribute(
            node,
            name,
        )
        .is_some()
    }

    pub fn set_attribute(
        &mut self,
        node: NodeId,
        name: impl Into<String>,
        value: impl Into<String>,
    ) {
        let Some(element) =
            self.element_mut(node)
        else {
            return;
        };

        if let Some(data) =
            element.element.as_mut()
        {
            data.attributes.set(
                name,
                value,
            );
        }
    }

    pub fn remove_attribute(
        &mut self,
        node: NodeId,
        name: &str,
    ) -> Option<Attribute> {
        let element =
            self.element_mut(node)?;

        element
            .element
            .as_mut()?
            .attributes
            .remove(name)
    }
}
