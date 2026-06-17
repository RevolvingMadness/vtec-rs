use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VtecFloodRecordStatus {
    NotApplicable,
    NotExpected,
    NearRecordOrRecordFloodExpected,
    FloodWithoutPeriodOfRecordToCompare,
}

impl FromStr for VtecFloodRecordStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "oo" => Self::NotApplicable,
            "no" => Self::NotExpected,
            "nr" => Self::NearRecordOrRecordFloodExpected,
            "uu" => Self::FloodWithoutPeriodOfRecordToCompare,

            _ => return Err(()),
        })
    }
}
