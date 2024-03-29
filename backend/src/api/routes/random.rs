use rand::Rng;
use serde_json::Value;
use std::{fs, string::String};
use tide::{prelude::*, Request, Result};
use crate::utils::{build_response, ResponseOptions};

#[derive(Deserialize)]
#[serde(default)]
struct Params {
    r#type: String,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            r#type: "any".to_string(),
        }
    }
}

fn get_random_seal(seal_type: String) -> std::result::Result<String, std::io::Error> {
    let seals = fs::read_dir("./assets/seals")?;

    let mut seals_vec = vec![];

    for seal in seals {
        seals_vec.push(seal?.path().into_os_string().into_string().unwrap());
    }

    seals_vec = seals_vec.into_iter().filter(|v| v.contains(&seal_type)).collect();

    Ok((&seals_vec[rand::thread_rng().gen_range(0..seals_vec.len())]).clone())
}

pub async fn handler(req: Request<()>) -> Result {
    let value_type = req.query::<Params>().unwrap().r#type;

    let res = match value_type.as_str() {
        "mp4" => {
            let seal = fs::read(get_random_seal("mp4".to_string())?)?;

            let resp = build_response::<Vec<u8>>(ResponseOptions {
                status: 200,
                content_type: "video/mp4",
                contents: seal,
            });

            Ok(resp)
        }

        "gif" => {
            let seal = fs::read(get_random_seal("gif".to_string())?)?;

            let resp = build_response::<Vec<u8>>(ResponseOptions {
                status: 200,
                content_type: "image/gif",
                contents: seal,
            });

            Ok(resp)
        }

        &_ => {
            let resp = build_response::<Value>(ResponseOptions {
                status: 400,
                content_type: "application/json",
                contents: json!({
                    "message": "invalid `type` query param"
                }),
            });

            Ok(resp)
        }
    };

    return res;
}
