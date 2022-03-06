use chrono::{Date, Datelike, TimeZone, Utc, Weekday};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt::Debug};

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
    pub fn weekday(&'a self) -> Result<Weekday, Box<dyn Error>> {
        Ok(self.to_chrono()?.weekday())
    }

    pub fn to_chrono(&'a self) -> Result<Date<Utc>, Box<dyn Error>> {
        let v = self.date.split(".").collect::<Vec<&str>>();
        let (day, month, year) = (
            v[0].parse::<u32>()?,
            v[1].parse::<u32>()?,
            v[2].parse::<i32>()?,
        );
        Ok(Utc.ymd(year, month, day))
    }
}
