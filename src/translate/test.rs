use super::*;

use url::ParseError;

#[test]
fn test_url_building() {
    let host = "https://api.example.com".to_string();
    let fqdn = translate::build_api_endpoint(host);
    assert_eq!(
        fqdn.unwrap().to_string(),
        "https://api.example.com/api/v1/translate"
    );
}

#[test]
fn test_url_building_bad_host() {
    let host = "heresAFakeString".to_string();
    let fqdn = translate::build_api_endpoint(host);

    assert_eq!(fqdn.unwrap_err(), ParseError::RelativeUrlWithoutBase);
}
