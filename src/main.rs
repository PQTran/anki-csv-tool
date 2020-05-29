extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate directories;
extern crate read_input;
extern crate tts_urls;
extern crate reqwest;
extern crate pinyin_zhuyin;

mod application_directory;
mod file_menu;
mod phonetics_settings;
mod audio_settings;
mod csv_reader_writer;
mod audio_asset_manager;

use application_directory::ApplicationFile;
use application_directory::DataDirectory;
use phonetics_settings::Phonetics;
use csv_reader_writer::Record;
use std::error::Error;
use std::process;

fn setup_application() -> Result<(), Box<dyn Error>> {
    application_directory::create_default_data_directories()?;
    Ok(())
}

fn select_input_csv_file() -> ApplicationFile {
    let csv_files = application_directory::get_input_csv_files();
    let selected_file = file_menu::select_from_file_menu(&csv_files);
    selected_file.clone()
}

fn select_phonetics_setting(records: &Vec<Record>) -> Phonetics {
    records[0].print_preview();
    phonetics_settings::select_from_phonetics_menu()
}

fn select_blacklist_audio_settings(records: &mut Vec<Record>) {
    audio_settings::select_from_blacklist_audio_menu(records);
}

fn download_audio_assets(records: &Vec<Record>) -> Result<(), Box<dyn Error>> {
    audio_asset_manager::download_audio_assets(records)?;

    Ok(())
}

fn create_output_csv(records: &mut Vec<Record>, phonetics_setting: Phonetics, input_file: &ApplicationFile) -> Result<(), Box<dyn Error>> {
    for record in records.iter_mut() {
        csv_reader_writer::update_record(record, phonetics_setting.clone());
    }

    let file_name = input_file.get_file_name();
    csv_reader_writer::write_to_output_csv(records, file_name)?;

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    setup_application()?;

    let csv_file = select_input_csv_file();
    let mut csv_records = csv_reader_writer::get_csv_records(&csv_file)?;

    let phonetics_setting = select_phonetics_setting(&csv_records);
    select_blacklist_audio_settings(&mut csv_records);
    download_audio_assets(&csv_records)?;
    create_output_csv(&mut csv_records, phonetics_setting, &csv_file)?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
