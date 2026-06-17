use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VtecIdentifier {
    Operational,
    Test,
    Experimental,
    ExperimentalOperational,
}

impl FromStr for VtecIdentifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "o" => Self::Operational,
            "t" => Self::Test,
            "e" => Self::Experimental,
            "x" => Self::ExperimentalOperational,

            _ => return Err(()),
        })
    }
}

impl VtecIdentifier {
    #[must_use]
    pub const fn is_operational(&self) -> bool {
        matches!(self, Self::Operational | Self::ExperimentalOperational)
    }

    #[must_use]
    pub const fn is_test(&self) -> bool {
        matches!(self, Self::Test)
    }

    #[must_use]
    pub const fn is_experimental(&self) -> bool {
        matches!(self, Self::Experimental | Self::ExperimentalOperational)
    }
}
