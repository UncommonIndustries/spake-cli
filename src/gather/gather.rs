// gather.rs
use reqwest::Client;
use std::error;

use crate::translate::translate;

pub async fn identify_strings_in_file(
    file_path: String,
    api_key: String,
    host: String,
) -> Result<String, Box<dyn error::Error>> {
    let client = Client::new();
    let fqdn = match translate::build_api_endpoint(host) {
        Ok(fqdn) => fqdn,
        Err(err) => {
            println!(
                "Error building API endpoint. Please check that your provided host isn't bogus."
            );
            return Err(err.into());
        }
    };

    let request = client
        .post(fqdn)
        .json("hello world")
        .header("X-API-KEY", api_key);
    let response = match request.send().await {
        Ok(res) => res,
        Err(err) => return Err(err.into()),
    };
    Ok("hello".to_owned())
}

// todo create a stringsRequestStructure.
