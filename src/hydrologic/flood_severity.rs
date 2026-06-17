use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VtecFloodSeverity {
    None,
    Zero,
    Minor,
    Moderate,
    Major,
    Unknown,
}

impl FromStr for VtecFloodSeverity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "n" => Self::None,
            "0" => Self::Zero,
            "1" => Self::Minor,
            "2" => Self::Moderate,
            "3" => Self::Major,
            "u" => Self::Unknown,

            _ => return Err(()),
        })
    }
}
