#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum QuirksMode {
    /// Standards mode.
    NoQuirks,

    /// Limited quirks mode.
    LimitedQuirks,

    /// Full quirks mode.
    Quirks,
}

impl QuirksMode {
    pub fn is_quirks(self) -> bool {
        matches!(
            self,
            Self::Quirks
        )
    }

    pub fn is_limited_quirks(
        self,
    ) -> bool {
        matches!(
            self,
            Self::LimitedQuirks
        )
    }

    pub fn is_no_quirks(
        self,
    ) -> bool {
        matches!(
            self,
            Self::NoQuirks
        )
    }
}

impl Default for QuirksMode {
    fn default() -> Self {
        Self::NoQuirks
    }
}
