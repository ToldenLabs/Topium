#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub namespace: Option<String>,
    pub prefix: Option<String>,
    pub local_name: String,
    pub value: String,
}

impl Attribute {
    pub fn new(
        local_name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            namespace: None,
            prefix: None,
            local_name: local_name.into(),
            value: value.into(),
        }
    }

    pub fn qualified_name(&self) -> String {
        match &self.prefix {
            Some(prefix) => {
                format!(
                    "{}:{}",
                    prefix,
                    self.local_name
                )
            }
            None => self.local_name.clone(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AttributeMap {
    attributes: Vec<Attribute>,
}

impl AttributeMap {
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }

    pub fn get(
        &self,
        name: &str,
    ) -> Option<&Attribute> {
        self.attributes
            .iter()
            .find(|attribute| {
                attribute.local_name == name
            })
    }

    pub fn get_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut Attribute> {
        self.attributes
            .iter_mut()
            .find(|attribute| {
                attribute.local_name == name
            })
    }

    pub fn set(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) {
        let name = name.into();
        let value = value.into();

        if let Some(attribute) =
            self.get_mut(&name)
        {
            /*
             * Setting the same value does not
             * constitute an attribute change.
             */
            if attribute.value == value {
                return;
            }

            attribute.value = value;
            return;
        }

        self.attributes.push(
            Attribute::new(name, value)
        );
    }

    pub fn remove(
        &mut self,
        name: &str,
    ) -> Option<Attribute> {
        let index =
            self.attributes
                .iter()
                .position(|attribute| {
                    attribute.local_name == name
                })?;

        Some(
            self.attributes.remove(index)
        )
    }

    pub fn contains(
        &self,
        name: &str,
    ) -> bool {
        self.get(name).is_some()
    }

    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Attribute> {
        self.attributes.iter()
    }
}
