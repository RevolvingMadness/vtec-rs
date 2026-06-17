use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VtecSignificance {
    Warning,
    Watch,
    Advisory,
    Statement,
    Forecast,
    Outlook,
    Synopsis,
}

impl FromStr for VtecSignificance {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "w" => Self::Warning,
            "a" => Self::Watch,
            "y" => Self::Advisory,
            "s" => Self::Statement,
            "f" => Self::Forecast,
            "o" => Self::Outlook,
            "n" => Self::Synopsis,

            _ => return Err(()),
        })
    }
}
