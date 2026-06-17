use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VtecImmediateCause {
    ExcessiveRainfall,
    Snowmelt,
    RainAndSnowmelt,
    DamOrLeveeFailure,
    GlacierDammedLakeOutburst,
    IceJam,
    RainAndOrSnowmeltAndOrIceJam,
    UpstreamFloodingPlusStormSurge,
    UpstreamFloodingPlusTidalEffects,
    ElevatedUpstreamFlowPlusTidalEffects,
    WindAndOrTidalEffects,
    UpstreamDamOrReservoirRelease,
    OtherMultipleCauses,
    OtherEffects,
    Unknown,
}

impl FromStr for VtecImmediateCause {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "er" => Self::ExcessiveRainfall,
            "sm" => Self::Snowmelt,
            "rs" => Self::RainAndSnowmelt,
            "dm" => Self::DamOrLeveeFailure,
            "go" => Self::GlacierDammedLakeOutburst,
            "ij" => Self::IceJam,
            "ic" => Self::RainAndOrSnowmeltAndOrIceJam,
            "fs" => Self::UpstreamFloodingPlusStormSurge,
            "ft" => Self::UpstreamFloodingPlusTidalEffects,
            "et" => Self::ElevatedUpstreamFlowPlusTidalEffects,
            "wt" => Self::WindAndOrTidalEffects,
            "dr" => Self::UpstreamDamOrReservoirRelease,
            "mc" => Self::OtherMultipleCauses,
            "ot" => Self::OtherEffects,
            "uu" => Self::Unknown,

            _ => return Err(()),
        })
    }
}
