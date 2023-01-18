// gather.rs
use base64::{engine::general_purpose, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{error, fs};

use url::Url;

const GATHER_API_ENDPOINT: &str = "api/v1/gather";

fn build_api_endpoint(host: String) -> Result<Url, Box<dyn error::Error>> {
    let base = Url::parse(&host)?;
    let joined = base.join(GATHER_API_ENDPOINT)?;
    Ok(joined)
}

pub async fn identify_strings_in_file(
    file_path: String,
    api_key: String,
    host: String,
) -> Result<String, Box<dyn error::Error>> {
    let client = Client::new();
    let fqdn = match build_api_endpoint(host) {
        Ok(fqdn) => fqdn,
        Err(err) => {
            println!(
                "Error building API endpoint. Please check that your provided host isn't bogus."
            );
            return Err(err.into());
        }
    };
    let request_body = match build_gather_request_object(file_path) {
        Ok(body) => body,
        Err(err) => return Err(err.into()),
    };
    let request = client
        .post(fqdn)
        .json(&request_body)
        .header("X-API-KEY", api_key);
    let response = match request.send().await {
        Ok(res) => res,
        Err(err) => return Err(err.into()),
    };

    match response.status() {
        reqwest::StatusCode::OK => (),
        _ => todo!(),
    }
    println!("{:?}", response);
    Ok("hello".to_owned())
}

// todo create a stringsRequestStructure.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct GatherRequestObject {
    file_text: String,
    file_path: String,
}

fn get_base64_encoded_string(data: String) -> String {
    let encoded_data = general_purpose::STANDARD.encode(data);
    return encoded_data;
}

fn build_gather_request_object(path: String) -> Result<GatherRequestObject, Box<dyn error::Error>> {
    let contents = match fs::read_to_string(path.clone()) {
        Ok(file_data) => file_data,
        Err(err) => return Err(Box::new(err)),
    };

    let object = GatherRequestObject {
        file_text: get_base64_encoded_string(contents),
        file_path: path,
    };
    Ok(object)
}
