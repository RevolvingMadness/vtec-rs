use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use thiserror::Error;

use crate::hydrologic::{
    flood_record_staturs::VtecFloodRecordStatus, flood_severity::VtecFloodSeverity,
    immediate_cause::VtecImmediateCause,
};

pub mod flood_record_staturs;
pub mod flood_severity;
pub mod immediate_cause;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HydrologicVtec {
    pub site_identifier: String,
    pub flood_severity: VtecFloodSeverity,
    pub immediate_cause: VtecImmediateCause,
    pub beginning_date: Option<DateTime<Utc>>,
    pub crest_date: Option<DateTime<Utc>>,
    pub ending_date: Option<DateTime<Utc>>,
    pub flood_record_status: VtecFloodRecordStatus,
}

#[derive(Debug, Error)]
pub enum HydrologicVtecParseError {
    #[error("Expected site identifier")]
    ExpectedSiteIdentifier,
    #[error("Invalid site identifier `{}`", .0)]
    InvalidSiteIdentifier(String),
    #[error("Expected flood severity")]
    ExpectedFloodSeverity,
    #[error("Invalid flood severity `{}`", .0)]
    InvalidFloodSeverity(String),
    #[error("Expected immediate cause")]
    ExpectedImmediateCause,
    #[error("Invalid immediate cause `{}`", .0)]
    InvalidImmediateCause(String),
    #[error("Expected beginning date")]
    ExpectedBeginningDate,
    #[error("Expected crest date")]
    ExpectedCrestDate,
    #[error("Expected ending date")]
    ExpectedEndingDate,
    #[error("Expected flood record status")]
    ExpectedFloodRecordStatus,
    #[error("Invalid flood record status `{}`", .0)]
    InvalidFloodRecordStatus(String),
}

impl FromStr for HydrologicVtec {
    type Err = HydrologicVtecParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_matches('/');

        let mut segments = s.split('.');

        let site_identifier = segments
            .next()
            .ok_or(HydrologicVtecParseError::ExpectedSiteIdentifier)?
            .to_owned();

        if site_identifier.len() != 5 {
            return Err(HydrologicVtecParseError::InvalidSiteIdentifier(
                site_identifier,
            ));
        }

        let flood_severity = segments
            .next()
            .ok_or(HydrologicVtecParseError::ExpectedFloodSeverity)?;

        let flood_severity = VtecFloodSeverity::from_str(flood_severity).map_err(|()| {
            HydrologicVtecParseError::InvalidFloodSeverity(flood_severity.to_owned())
        })?;

        let immediate_cause = segments
            .next()
            .ok_or(HydrologicVtecParseError::ExpectedImmediateCause)?;

        let immediate_cause = VtecImmediateCause::from_str(immediate_cause).map_err(|()| {
            HydrologicVtecParseError::InvalidImmediateCause(immediate_cause.to_owned())
        })?;

        let beginning_date = segments
            .next()
            .ok_or(HydrologicVtecParseError::ExpectedBeginningDate)?;

        let beginning_date = NaiveDateTime::parse_from_str(beginning_date, "%y%m%dT%H%MZ")
            .ok()
            .map(|date_time| Utc.from_local_datetime(&date_time).unwrap());

        let crest_date = segments
            .next()
            .ok_or(HydrologicVtecParseError::ExpectedCrestDate)?;

        let crest_date = NaiveDateTime::parse_from_str(crest_date, "%y%m%dT%H%MZ")
            .ok()
            .map(|date_time| Utc.from_local_datetime(&date_time).unwrap());

        let ending_date = segments
            .next()
            .ok_or(HydrologicVtecParseError::ExpectedEndingDate)?;

        let ending_date = NaiveDateTime::parse_from_str(ending_date, "%y%m%dT%H%MZ")
            .ok()
            .map(|date_time| Utc.from_local_datetime(&date_time).unwrap());

        let flood_record_status = segments
            .next()
            .ok_or(HydrologicVtecParseError::ExpectedFloodRecordStatus)?;

        let flood_record_status =
            VtecFloodRecordStatus::from_str(flood_record_status).map_err(|()| {
                HydrologicVtecParseError::InvalidFloodRecordStatus(flood_record_status.to_owned())
            })?;

        Ok(Self {
            site_identifier,
            flood_severity,
            immediate_cause,
            beginning_date,
            crest_date,
            ending_date,
            flood_record_status,
        })
    }
}
