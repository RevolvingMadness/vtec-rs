use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VtecExtension {
    Area,
    Time,
    Both,
}

impl FromStr for VtecExtension {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "exa" => Self::Area,
            "ext" => Self::Time,
            "exb" => Self::Both,

            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VtecAction {
    New,
    Continued,
    Extended(VtecExtension),
    Upgraded,
    Cancelled,
    Expired,
    Routine,
    Correction,
}

impl FromStr for VtecAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "new" => Self::New,
            "con" => Self::Continued,
            "exa" | "ext" | "exb" => Self::Extended(VtecExtension::from_str(s).unwrap()),
            "upg" => Self::Upgraded,
            "can" => Self::Cancelled,
            "exp" => Self::Expired,
            "rou" => Self::Routine,
            "cor" => Self::Correction,

            _ => return Err(()),
        })
    }
}
