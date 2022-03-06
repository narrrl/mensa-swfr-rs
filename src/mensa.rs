use chrono::{Weekday, DateTime, Utc, TimeZone, Date, Datelike};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, error::Error};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "plan")]
pub struct Plan {
    #[serde(rename = "ort")]
    place: Place,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "ort")]
pub struct Place {
    id: String,
    mensa: Mensa,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "mensa")]
pub struct Mensa {
    #[serde(rename = "$value")]
    name: String,
    #[serde(rename = "tagesplan")]
    days: Vec<Day>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "tagesplan")]
pub struct Day {
    #[serde(rename = "datum")]
    date: String,
    #[serde(rename = "menue")]
    menues: Vec<Menu>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "menue")]
pub struct Menu {
    art: String,
    #[serde(rename = "zusatz")]
    food_type: String,
    name: String,
    #[serde(rename = "preis")]
    price: Price,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "preis")]
pub struct Price {
    #[serde(rename = "studierende")]
    price_students: String,
    #[serde(rename = "angestellte")]
    price_workers: String,
    #[serde(rename = "gaeste")]
    price_guests: String,
    #[serde(rename = "schueler")]
    price_school: String,
}

impl<'a> Plan {
    pub fn day(&'a self, day: Weekday) -> Option<&'a Day> {
        self.place.mensa.days.get(day as usize)
    }

    pub fn days(&'a self) -> Vec<&'a Day> {
        self.place.mensa.days.iter().collect()
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
        let (day, month, year) = (v[0].parse::<u32>()?, v[1].parse::<u32>()?, v[2].parse::<i32>()?);
        Ok(Utc.ymd(year, month, day))
    }
}
