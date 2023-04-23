use std::collections::HashMap;
use url::Url;

use load_dotenv::load_dotenv;

load_dotenv!();

pub fn get_github_url(from: Option<&str>) -> String {
    let client_id = std::env!("GITHUB_OAUTH_CLIENT_ID");
    let redirect_uri = std::env!("GITHUB_OAUTH_REDIRECT_URL");

    let root_url = "https://github.com/login/oauth/authorize";
    let mut options = HashMap::new();
    options.insert("redirect_uri", redirect_uri);
    options.insert("client_id", client_id);
    options.insert("scope", "user:email");
    options.insert("state", from.unwrap_or_default());

    let url = Url::parse_with_params(root_url, &options).unwrap();
    let qs = url.query().unwrap();

    format!("{}?{}", root_url, qs)
}
