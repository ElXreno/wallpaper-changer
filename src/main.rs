#![windows_subsystem = "windows"]

pub mod structs;
pub mod utils;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate wallpaper;
extern crate online;

use std::thread::sleep;
use std::time::Duration;

const BASE_PATH: &'static str = "https://smnrer.lh1.in/wallpapers";

fn main() {
    let sleep_time = 30;
    let mut current_version = 0;

    println!("Config: sleep time: '{}'; current version: '{}'", sleep_time, current_version);

    loop {
        if online::online(None).unwrap_or(false) {
            println!("Fetching info from server...");

            let wallpapers_response = reqwest::blocking::get(&format!("{}/wallpapers.json", BASE_PATH))
                .unwrap()
                .text()
                .unwrap();

            let wallpapers: structs::Wallpapers = serde_json::from_str(&wallpapers_response).unwrap();

            println!("Current version: {}", current_version);
            println!("Wallpaper response dump: {:?}", wallpapers);

            if current_version != wallpapers.version {
                let download_url;

                if wallpapers.download_url.starts_with("http://")
                    || wallpapers.download_url.starts_with("https://")
                {
                    download_url = wallpapers.download_url;
                } else {
                    download_url = format!("{}/{}", BASE_PATH, wallpapers.download_url);
                }

                println!("Setting up wallpaper from url: {}", download_url);

                wallpaper::set_from_url(&download_url).unwrap();

                current_version = wallpapers.version;
            }
        } else {
            println!("No internet connection!");
        }

        println!("Sleeping {} seconds", sleep_time);

        sleep(Duration::from_secs(sleep_time));
    }
}