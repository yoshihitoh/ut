use std::fmt::Debug;

use chrono::{Date, TimeZone};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use thiserror::Error;

use crate::find::{FindByName, FindError, PossibleNames, PossibleValues};
use crate::provider::DateTimeProvider;
use crate::validate::IntoValidationError;

#[derive(Error, Debug, PartialEq)]
pub enum PresetError {
    #[error("Wrong preset. error:{0}")]
    WrongName(FindError),
}

impl From<FindError> for PresetError {
    fn from(e: FindError) -> Self {
        PresetError::WrongName(e)
    }
}

impl IntoValidationError for PresetError {
    fn into_validation_error(self) -> String {
        use PresetError::*;
        match &self {
            WrongName(e) => match e {
                FindError::NotFound => {
                    let names = Preset::possible_names();
                    format!("{} possible names: [{}]", self, names.join(", "))
                }
                _ => format!("{}", self),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, EnumString, Display)]
pub enum Preset {
    #[strum(serialize = "today")]
    Today,

    #[strum(serialize = "tomorrow")]
    Tomorrow,

    #[strum(serialize = "yesterday")]
    Yesterday,
}

impl Preset {
    pub fn as_date<P, Tz>(self, provider: &P) -> Date<Tz>
    where
        P: DateTimeProvider<Tz>,
        Tz: TimeZone + Debug,
    {
        match self {
            Preset::Today => provider.today(),
            Preset::Tomorrow => provider.tomorrow(),
            Preset::Yesterday => provider.yesterday(),
        }
    }
}

impl PossibleValues for Preset {
    type Iterator = PresetIter;

    fn possible_values() -> Self::Iterator {
        Preset::iter()
    }
}

impl PossibleNames for Preset {}

impl FindByName for Preset {
    type Error = PresetError;
}
