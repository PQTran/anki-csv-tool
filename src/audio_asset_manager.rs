use crate::models::{Record, DataDirectory};
use crate::application_directory;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use colored::*;

fn _print_download_success_msg(character: &str, file_name: &str) {
    println!("Downloaded: {} as: {}", character, file_name);
}

fn _print_download_failed_msg(url: &str) {
    println!("Download failed with url: {}", url.red());
}

fn _download(url: &str, output_path: &Path, replace_file: bool) -> Result<(), Box<dyn Error>> {
    let mut body = reqwest::blocking::get(url)?;

    let mut buffer = File::create(output_path)?;
    body.copy_to(&mut buffer)?;

    Ok(())
}

pub fn download_audio_assets(records: &Vec<Record>) {
    const LANG: &str = "zh-TW";

    // let output_path = application_directory::get_data_directory_path(DataDirectory::OutputAudio);
    let output_path = application_directory::get_anki_audio_path();

    for record in records.iter().filter(|x| x.get_download_audio()) {
        let mut output_path = output_path.clone();
        output_path.push(record.get_audio_file_name());

        let asset_url = tts_urls::google_translate::url(record.get_data().get_character(), LANG);

        if !output_path.is_file() {
            if let Ok(_) = _download(&asset_url, &output_path, false) {
                _print_download_success_msg(record.get_data().get_character(), record.get_audio_file_name());
            } else {
                _print_download_failed_msg(&asset_url);
            }
        }
    }
}
