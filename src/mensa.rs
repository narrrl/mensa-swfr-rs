use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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

pub enum Weekday {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
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
