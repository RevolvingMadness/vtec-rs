use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use thiserror::Error;

use crate::primary::{
    action::VtecAction, identifier::VtecIdentifier, phenomenon::VtecPhenomenon,
    significance::VtecSignificance,
};

pub mod action;
pub mod identifier;
pub mod phenomenon;
pub mod significance;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrimaryVtec {
    pub identifier: VtecIdentifier,
    pub action: VtecAction,
    pub office_id: String,
    pub phenomenon: VtecPhenomenon,
    pub significance: VtecSignificance,
    pub tracking_number: u16,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Error)]
pub enum PrimaryVtecParseError {
    #[error("Expected identifier")]
    ExpectedIdentifier,
    #[error("Invalid identifier `{}`", .0)]
    InvalidIdentifier(String),
    #[error("Expected action")]
    ExpectedAction,
    #[error("Invalid action `{}`", .0)]
    InvalidAction(String),
    #[error("Expected office id")]
    ExpectedOfficeId,
    #[error("Invalid office id `{}`", .0)]
    InvalidOfficeId(String),
    #[error("Expected phenomenon")]
    ExpectedPhenomenon,
    #[error("Invalid phenomenon `{}`", .0)]
    InvalidPhenomenon(String),
    #[error("Expected significance")]
    ExpectedSignificance,
    #[error("Invalid significance `{}`", .0)]
    InvalidSignificance(String),
    #[error("Expected tracking number")]
    ExpectedTrackingNumber,
    #[error("Invalid tracking number `{}`", .0)]
    InvalidTrackingNumber(String),
    #[error("Expected start and end dates")]
    ExpectedStartAndEndDates,
    #[error("Expected start date")]
    ExpectedStartDate,
    #[error("Invalid start date `{}`", .0)]
    InvalidStartDate(String),
    #[error("Expected end date")]
    ExpectedEndDate,
    #[error("Invalid end date `{}`", .0)]
    InvalidEndDate(String),
}

impl FromStr for PrimaryVtec {
    type Err = PrimaryVtecParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_matches('/');

        let mut segments = s.split('.');

        let identifer = segments
            .next()
            .ok_or(PrimaryVtecParseError::ExpectedIdentifier)?;

        let identifier = VtecIdentifier::from_str(identifer)
            .map_err(|()| PrimaryVtecParseError::InvalidIdentifier(identifer.to_owned()))?;

        let action = segments
            .next()
            .ok_or(PrimaryVtecParseError::ExpectedAction)?;

        let action = VtecAction::from_str(action)
            .map_err(|()| PrimaryVtecParseError::InvalidAction(action.to_owned()))?;

        let office_id = segments
            .next()
            .ok_or(PrimaryVtecParseError::ExpectedOfficeId)?
            .to_owned();

        if office_id.len() != 4 {
            return Err(PrimaryVtecParseError::InvalidOfficeId(office_id));
        }

        let phenomenon = segments
            .next()
            .ok_or(PrimaryVtecParseError::ExpectedPhenomenon)?;

        let phenomenon = VtecPhenomenon::from_str(phenomenon)
            .map_err(|()| PrimaryVtecParseError::InvalidPhenomenon(phenomenon.to_owned()))?;

        let significance = segments
            .next()
            .ok_or(PrimaryVtecParseError::ExpectedSignificance)?;

        let significance = VtecSignificance::from_str(significance)
            .map_err(|()| PrimaryVtecParseError::InvalidSignificance(significance.to_owned()))?;

        let tracking_number = segments
            .next()
            .ok_or(PrimaryVtecParseError::ExpectedTrackingNumber)?;

        let tracking_number = tracking_number.parse::<u16>().map_err(|_| {
            PrimaryVtecParseError::InvalidTrackingNumber(tracking_number.to_owned())
        })?;

        let mut times = segments
            .next()
            .ok_or(PrimaryVtecParseError::ExpectedStartAndEndDates)?
            .split('-');

        let start_date = times
            .next()
            .ok_or(PrimaryVtecParseError::ExpectedStartDate)?;

        let start_date = NaiveDateTime::parse_from_str(start_date, "%y%m%dT%H%MZ")
            .ok()
            .map(|date_time| Utc.from_local_datetime(&date_time).unwrap());

        let end_date = times.next().ok_or(PrimaryVtecParseError::ExpectedEndDate)?;

        let end_date = NaiveDateTime::parse_from_str(end_date, "%y%m%dT%H%MZ")
            .ok()
            .map(|date_time| Utc.from_local_datetime(&date_time).unwrap());

        Ok(Self {
            identifier,
            action,
            office_id,
            phenomenon,
            significance,
            tracking_number,
            start_date,
            end_date,
        })
    }
}
