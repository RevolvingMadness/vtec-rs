use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

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

impl Display for VtecSignificance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::Warning => "Warning",
            Self::Watch => "Watch",
            Self::Advisory => "Advisory",
            Self::Statement => "Statement",
            Self::Forecast => "Forecast",
            Self::Outlook => "Outlook",
            Self::Synopsis => "Synopsis",
        };

        f.write_str(string)
    }
}
