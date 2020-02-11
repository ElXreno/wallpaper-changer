pub mod structs;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate winapi;

use std::thread::sleep;
use std::time::Duration;
use std::path::Path;
use std::fs::File;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::error::Error;
use std::os::raw::c_void;
use std::iter;
use std::os::windows::ffi::OsStrExt;
use std::io;
use winapi::um::winuser::SystemParametersInfoW;
use winapi::um::winuser::SPIF_SENDCHANGE;
use winapi::um::winuser::SPIF_UPDATEINIFILE;
use winapi::um::winuser::SPI_SETDESKWALLPAPER;

const BASE_PATH: &'static str = "https://smnrer.lh1.in/wallpapers";
const WALLPAPERS_DIR: &'static str = "C:\\wallpapers";

fn main() {
    let sleep_time = 60;
    let mut last_version = 0;
    loop {
        let wallpapers_response =
            reqwest::blocking::get(&format!("{}/wallpapers.json", BASE_PATH))
                .unwrap()
                .text()
                .unwrap();

        let wallpapers: structs::Wallpapers = serde_json::from_str(&wallpapers_response).unwrap();

        if last_version == wallpapers.version {
            continue;
        }
        
        let download_url;

        if wallpapers.download_url.starts_with("http://") || wallpapers.download_url.starts_with("https://") {
            download_url = wallpapers.download_url;
        } else {
            download_url = format!("{}/{}", BASE_PATH, wallpapers.download_url);
        }

        let destination_file = format!("{}\\wallpaper-{}.{}", WALLPAPERS_DIR, wallpapers.version, Path::new(&download_url).extension().and_then(OsStr::to_str).unwrap());

        create_dir(&PathBuf::from(&WALLPAPERS_DIR));

        let mut file = { File::create(&destination_file).unwrap() };

        reqwest::blocking::get(&download_url).unwrap().copy_to(&mut file).unwrap();

        set_from_path(&destination_file);
        
        last_version = wallpapers.version;
        
        sleep(Duration::from_secs(sleep_time));
    }
}

pub fn create_dir(path: &PathBuf) {
    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(()) => println!("{} dir created successfully!", &path.display()),
            Err(e) => panic!("Error {}", e.description()),
        }
    } else if !path.is_dir() {
        panic!(
            "{} already exists and is not a directory, exiting.",
            &path.display()
        );
    }
}

pub fn set_from_path(path: &str) {
    unsafe {
        let path = OsStr::new(path)
            .encode_wide()
            // append null byte
            .chain(iter::once(0))
            .collect::<Vec<u16>>();
        let successful = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        ) == 1;
    }
}
