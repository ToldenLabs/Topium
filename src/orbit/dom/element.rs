use super::attribute::AttributeMap;
use super::node::NodeId;

#[derive(Debug)]
pub struct ElementData {
    pub local_name: String,
    pub namespace: String,
    pub attributes: AttributeMap,
    pub node_id: NodeId,
}

impl ElementData {
    pub fn new(
        node_id: NodeId,
        local_name: impl Into<String>,
    ) -> Self {
        Self {
            node_id,
            local_name: local_name.into(),
            namespace:
                "http://www.w3.org/1999/xhtml"
                    .to_string(),
            attributes:
                AttributeMap::new(),
        }
    }

    pub fn tag_name(&self) -> &str {
        &self.local_name
    }

    pub fn get_attribute(
        &self,
        name: &str,
    ) -> Option<&str> {
        self.attributes
            .get(name)
            .map(|attribute| {
                attribute.value.as_str()
            })
    }

    pub fn has_attribute(
        &self,
        name: &str,
    ) -> bool {
        self.attributes.contains(name)
    }

    pub fn set_attribute(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.attributes.set(
            name,
            value,
        );
    }

    pub fn remove_attribute(
        &mut self,
        name: &str,
    ) {
        self.attributes.remove(name);
    }
}
