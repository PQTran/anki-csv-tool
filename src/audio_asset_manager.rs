use crate::csv_reader_writer::Record;
use crate::application_directory::DataDirectory;
use crate::application_directory;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

fn get_output_mp3_path(path: &PathBuf, mp3_name: &str) -> PathBuf {
    let mut output_path = path.clone();
    output_path.push(format!("{}.mp3", mp3_name));
    output_path
}

pub fn download_audio_assets(records: &Vec<Record>) -> Result<(), Box<dyn Error>> {
    const LANG: &str = "zh-TW";

    let output_path = application_directory::get_data_directory_path(DataDirectory::OutputAudio);

    let audio_asset_records: Vec<&Record> = records.iter().filter(|x| x.get_download_audio()).collect();
    for record in audio_asset_records.iter() {
        let output_path = get_output_mp3_path(&output_path, record.get_pronunciation());
        if !output_path.as_path().is_file() {
            let asset_url = tts_urls::google_translate::url(record.get_character(), LANG);
            let mut body = reqwest::blocking::get(&asset_url)?;


            let mut buffer = File::create(output_path)?;
            body.copy_to(&mut buffer)?;
            println!("Downloaded: {}.mp3", record.get_character());
        }
    }

    Ok(())
}
