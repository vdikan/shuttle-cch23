use serde::{Deserialize, Serialize};
use serde_json::json;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum::extract::Json;

#[derive(Serialize, Deserialize)]
pub struct Reindeer {
    name: String,
    strength: u32,
}

fn strength(v: &[Reindeer]) -> Option<u32> {
    let strength = v.iter().map(|a| a.strength).reduce(|a, b| a + b);
    strength
}

pub async fn day_04_strength(Json(payload): Json<Vec<Reindeer>>) -> Response {
    let strength = strength(&payload);
    match strength {
        Some(num) => format!("{}", num).into_response(),
        None => StatusCode::BAD_REQUEST.into_response(),
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReindeerExtended {
    name: String,
    strength: i32,
    speed: f64,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

fn find_fastest(v: &[ReindeerExtended]) -> Option<&ReindeerExtended> {
    let fastest = v
        .iter()
        .reduce(|a, b| if a.speed > b.speed { a } else { b });
    fastest
}

fn find_tallest(v: &[ReindeerExtended]) -> Option<&ReindeerExtended> {
    let tallest = v
        .iter()
        .reduce(|a, b| if a.height > b.height { a } else { b });
    tallest
}

fn find_magician(v: &[ReindeerExtended]) -> Option<&ReindeerExtended> {
    let magician = v.iter().reduce(|a, b| {
        if a.snow_magic_power > b.snow_magic_power {
            a
        } else {
            b
        }
    });
    magician
}

fn find_consumer(v: &[ReindeerExtended]) -> Option<&ReindeerExtended> {
    let consumer = v.iter().reduce(|a, b| {
        if a.candies_eaten_yesterday > b.candies_eaten_yesterday {
            a
        } else {
            b
        }
    });
    consumer
}

fn analyze_contest(v: &[ReindeerExtended]) -> Option<serde_json::Value> {
    if let (Some(fastest), Some(tallest), Some(magician), Some(consumer)) = (
        find_fastest(v),
        find_tallest(v),
        find_magician(v),
        find_consumer(v),
    ) {
        Some(json!({
            "fastest": format!("Speeding past the finish line with a strength of {} is {}",
                               fastest.strength, fastest.name),
            "tallest": format!("{} is standing tall with his {} cm wide antlers",
                               tallest.name, tallest.antler_width),
            "magician": format!("{} could blast you away with a snow magic power of {}",
                                magician.name, magician.snow_magic_power),
            "consumer": format!("{} ate lots of candies, but also some {}",
                                consumer.name, consumer.favorite_food)
        }))
    } else {
        None
    }
}

pub async fn day_04_contest(Json(payload): Json<Vec<ReindeerExtended>>) -> Json<serde_json::Value> {
    let contest_result = analyze_contest(&payload).unwrap();
    axum::Json(contest_result)
}

#[cfg(test)]
mod tests {
    use super::Reindeer;
    use super::strength;
    use serde_json::Result;

    #[test]
    fn test_deserialize() -> Result<()> {
        let data = r#"
[
    { "name": "Dasher", "strength": 5 },
    { "name": "Dancer", "strength": 6 },
    { "name": "Prancer", "strength": 4 },
    { "name": "Vixen", "strength": 7 }
]"#;
        let parsed_v: Vec<Reindeer> = serde_json::from_str(data)?;
        assert_eq!(parsed_v[1].strength, 6);
        assert_eq!(parsed_v[2].name, "Prancer".to_string());

        Ok(())
    }

    #[test]
    fn test_strength() -> Result<()> {
        let data = r#"
[
    { "name": "Dasher", "strength": 5 },
    { "name": "Dancer", "strength": 6 },
    { "name": "Prancer", "strength": 4 },
    { "name": "Vixen", "strength": 7 }
]"#;
        let parsed_v: Vec<Reindeer> = serde_json::from_str(data)?;
        let strength = strength(&parsed_v);
        assert_eq!(strength.unwrap(), 22);

        Ok(())
    }
}
