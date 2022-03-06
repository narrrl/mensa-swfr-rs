use error::MensaError;
use mensa::Plan;

use reqwest::{StatusCode, Url};

pub mod error;
pub mod mensa;

pub async fn get_week_rampart(key: &str) -> Result<Plan, MensaError> {
    let mut url = Url::parse("https://www.swfr.de/index.php").unwrap();
    let query = format!(
        "id=1400&type=98&tx_swfrspeiseplan_pi1[apiKey]={}&tx_swfrspeiseplan_pi1[ort]=610",
        key
    );
    url.set_query(Some(&query));
    let url_string = url.to_string();

    let res = match reqwest::get(&url_string).await {
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

    println!("{}", body);
    let plan: Plan = quick_xml::de::from_str(&body)?;

    Ok(plan)
}
