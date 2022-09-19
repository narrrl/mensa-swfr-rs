use chrono::{Date, Datelike, TimeZone, Utc, Weekday as ChronoWeekday};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom, error::Error, fmt::Debug};
use strum_macros::EnumIter;

use crate::error::MensaError;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "plan")]
pub struct Plan {
    #[serde(rename = "ort")]
    pub place: Place,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "ort")]
pub struct Place {
    pub id: String,
    pub mensa: Mensa,
    #[serde(rename = "tagesplan")]
    days: Vec<Day>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "mensa")]
pub struct Mensa {
    #[serde(rename = "$value")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "tagesplan")]
pub struct Day {
    #[serde(rename = "datum")]
    date: String,
    #[serde(rename = "menue")]
    pub menues: Vec<Menu>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "menue")]
pub struct Menu {
    pub art: String,
    #[serde(rename = "zusatz")]
    pub food_type: Option<String>,
    pub name: String,
    #[serde(rename = "preis")]
    pub price: Price,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "preis")]
pub struct Price {
    #[serde(rename = "studierende")]
    pub price_students: String,
    #[serde(rename = "angestellte")]
    pub price_workers: String,
    #[serde(rename = "gaeste")]
    pub price_guests: String,
    #[serde(rename = "schueler")]
    pub price_school: String,
}

impl<'a> Plan {
    pub fn day(&'a self, day: Weekday) -> Option<&'a Day> {
        self.days().get(&day).map(|d| *d)
    }

    pub fn days(&'a self) -> HashMap<Weekday, &'a Day> {
        self.place
            .days
            .iter()
            .map(|d| match d.weekday() {
                Ok(w) => Some((w, d)),
                Err(_) => None,
            })
            .filter_map(|x| x)
            .collect::<HashMap<Weekday, &'a Day>>()
    }

    pub fn mensa_name(&'a self) -> &'a str {
        &self.place.mensa.name
    }
}

impl<'a> Day {
    pub fn weekday(&'a self) -> Result<Weekday, Box<dyn Error + Send + Sync>> {
        Ok(self.to_chrono()?.weekday().into())
    }

    pub fn to_chrono(&'a self) -> Result<Date<Utc>, Box<dyn Error + Send + Sync>> {
        let v = self.date.split(".").collect::<Vec<&str>>();
        let (day, month, year) = (
            v[0].parse::<u32>()?,
            v[1].parse::<u32>()?,
            v[2].parse::<i32>()?,
        );
        Ok(Utc.ymd(year, month, day))
    }
}

#[derive(PartialEq, EnumIter, Hash, Eq, Clone, Copy)]
pub enum Weekday {
    /// Monday.
    Mon = 0,
    /// Tuesday.
    Tue = 1,
    /// Wednesday.
    Wed = 2,
    /// Thursday.
    Thu = 3,
    /// Friday.
    Fri = 4,
    /// Saturday.
    Sat = 5,
    /// Sunday.
    Sun = 6,
}

impl From<ChronoWeekday> for Weekday {
    fn from(chrono_weekday: ChronoWeekday) -> Weekday {
        match chrono_weekday {
            ChronoWeekday::Mon => Weekday::Mon,
            ChronoWeekday::Tue => Weekday::Tue,
            ChronoWeekday::Wed => Weekday::Wed,
            ChronoWeekday::Thu => Weekday::Thu,
            ChronoWeekday::Fri => Weekday::Fri,
            ChronoWeekday::Sat => Weekday::Sat,
            ChronoWeekday::Sun => Weekday::Sun,
        }
    }
}

impl TryFrom<&str> for Weekday {
    type Error = MensaError;
    fn try_from(value: &str) -> Result<Weekday, MensaError> {
        match String::from(value).to_lowercase().as_str() {
            "mon" => Ok(Weekday::Mon),
            "tue" => Ok(Weekday::Tue),
            "wed" => Ok(Weekday::Wed),
            "thu" => Ok(Weekday::Thu),
            "fri" => Ok(Weekday::Fri),
            "sat" => Ok(Weekday::Sat),
            "sun" => Ok(Weekday::Sun),
            _ => Err(MensaError::ParseWeekdayError),
        }
    }
}
