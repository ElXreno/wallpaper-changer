extern crate serde;

use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct Wallpapers {
    pub download_url: String,
    pub fallback_url: String,
    pub version: i16,
}
