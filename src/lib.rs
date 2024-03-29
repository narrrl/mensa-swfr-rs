use std::{collections::HashMap, fmt};

use error::MensaError;
use mensa::Plan;

use reqwest::{StatusCode, Url};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub mod error;
pub mod mensa;

pub const API_URL: &'static str = "https://www.swfr.de/apispeiseplan";
pub const DEFAULT_QUERY: &'static str = "type=98";

pub async fn request_all(key: &str) -> Result<HashMap<MensaPlace, Plan>, MensaError> {
    let mut map = HashMap::new();
    for place in MensaPlace::iter() {
        let plan = request(UrlBuilder::new(key).set_place(&place)).await?;
        map.insert(place, plan);
    }
    Ok(map)
}

pub async fn request_mensa(place: &MensaPlace, key: &str) -> Result<Plan, MensaError> {
    request(UrlBuilder::new(key).set_place(place)).await
}

pub async fn request(url: &mut UrlBuilder) -> Result<Plan, MensaError> {
    let url = url.build();
    let res = match reqwest::get(&url).await {
        Ok(res) => res,
        Err(why) => {
            return Err(MensaError::from(format!(
                "failed to send api request: {:?}",
                why
            )));
        }
    };

    let status = res.status();

    let body = match res.text().await {
        Ok(body) => body,
        Err(why) => {
            return Err(MensaError::from(format!("failed to get body: {:?}", why)));
        }
    };
    if !status.eq(&StatusCode::OK) {
        return Err(MensaError::from(format!(
            "status code not OK: {:?}",
            status
        )));
    }

    let plan: Plan = quick_xml::de::from_str(&body)?;

    Ok(plan)
}

pub struct UrlBuilder {
    api_key: String,
    url: Url,
    query: String,
}

impl UrlBuilder {
    pub fn new(api_key: &str) -> UrlBuilder {
        UrlBuilder {
            api_key: String::from(api_key),
            url: Url::parse(API_URL).unwrap(),
            query: String::from(DEFAULT_QUERY),
        }
    }

    pub fn new_empty_query(api_key: &str) -> UrlBuilder {
        UrlBuilder {
            api_key: String::from(api_key),
            url: Url::parse(API_URL).unwrap(),
            query: String::new(),
        }
    }

    pub fn set_place<'a>(&'a mut self, place: &MensaPlace) -> &'a mut UrlBuilder {
        self.add_query_para("tx_speiseplan_pi1[ort]", &place.id());
        self
    }

    pub fn add_query_para<'a>(&'a mut self, name: &str, value: &str) -> &'a mut UrlBuilder {
        if self.query.is_empty() {
            self.query = format!("{}={}", name, value);
        } else {
            self.query = format!("{}&{}={}", self.query, name, value);
        }
        self
    }

    pub fn build(&mut self) -> String {
        self.add_query_para(
            "tx_speiseplan_pi1[apiKey]",
            String::from(&self.api_key).as_str(),
        );
        self.url.set_query(Some(&self.query));
        self.url.to_string()
    }
}

#[derive(PartialEq, EnumIter, Hash, Eq, Clone, Copy)]
pub enum MensaPlace {
    Rempartstraße,
    Institutsviertel,
    Littenweiler,
    Flugplatz,
    Furtwangen,
    Offenburg,
    Gengenbach,
    Kehl,
    Schwenningen,
    Lörrach,
    MusiKantine,
    OttoHahnFurtwangen,
    Trossingen,
}

impl MensaPlace {
    pub fn id(&self) -> String {
        match self {
            MensaPlace::Rempartstraße => String::from("610"),
            MensaPlace::Institutsviertel => String::from("620"),
            MensaPlace::Littenweiler => String::from("630"),
            MensaPlace::Furtwangen => String::from("641"),
            MensaPlace::Offenburg => String::from("651"),
            MensaPlace::Gengenbach => String::from("652"),
            MensaPlace::Flugplatz => String::from("681"),
            MensaPlace::Kehl => String::from("661"),
            MensaPlace::Schwenningen => String::from("671"),
            MensaPlace::Lörrach => String::from("677"),
            MensaPlace::MusiKantine => String::from("722"),
            MensaPlace::OttoHahnFurtwangen => String::from("9012"),
            MensaPlace::Trossingen => String::from("9019"),
        }
    }
}

impl TryFrom<&str> for MensaPlace {
    type Error = MensaError;
    fn try_from(value: &str) -> Result<MensaPlace, MensaError> {
        match value {
            "610" => Ok(MensaPlace::Rempartstraße),
            "620" => Ok(MensaPlace::Institutsviertel),
            "630" => Ok(MensaPlace::Littenweiler),
            "681" => Ok(MensaPlace::Flugplatz),
            "641" => Ok(MensaPlace::Furtwangen),
            "651" => Ok(MensaPlace::Offenburg),
            "652" => Ok(MensaPlace::Gengenbach),
            "661" => Ok(MensaPlace::Kehl),
            "671" => Ok(MensaPlace::Schwenningen),
            "677" => Ok(MensaPlace::Lörrach),
            "722" => Ok(MensaPlace::MusiKantine),
            "9012" => Ok(MensaPlace::OttoHahnFurtwangen),
            "9019" => Ok(MensaPlace::Trossingen),
            _ => Err(MensaError::ParseMensaPlaceError),
        }
    }
}

impl fmt::Display for MensaPlace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            MensaPlace::Rempartstraße => "Rempartstraße",
            MensaPlace::Institutsviertel => "Institutsviertel",
            MensaPlace::Littenweiler => "Littenweiler",
            MensaPlace::Flugplatz => "Flugplatz",
            MensaPlace::Furtwangen => "Furtwangen",
            MensaPlace::Offenburg => "Offenburg",
            MensaPlace::Gengenbach => "Gengenbach",
            MensaPlace::Kehl => "Kehl",
            MensaPlace::Schwenningen => "Schwenningen",
            MensaPlace::Lörrach => "Lörrach",
            MensaPlace::MusiKantine => "MusiKantine",
            MensaPlace::OttoHahnFurtwangen => "Otto-Hahn-Gymnasium Furtwangen",
            MensaPlace::Trossingen => "Trossingen",
        })
    }
}
