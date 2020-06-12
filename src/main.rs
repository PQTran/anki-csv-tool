#![allow(warnings)]

extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate directories;
extern crate read_input;
extern crate tts_urls;
extern crate reqwest;
extern crate zhuyin;
extern crate colored;

mod models;
mod application_directory;
mod file_menu;
mod phonetics_settings;
mod audio_settings;
mod csv_reader_writer;
mod audio_asset_manager;

use models::{ApplicationFile, Record};
use phonetics_settings::Phonetics;
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

fn select_output_phonetics_setting(records: &Vec<Record>) -> Phonetics {
    phonetics_settings::select_from_output_phonetics_menu()
}

fn select_blacklist_audio_settings(records: &mut Vec<Record>) {
    audio_settings::select_from_blacklist_audio_menu(records);
}

fn download_audio_assets(records: &mut Vec<Record>) {
    audio_asset_manager::download_audio_assets(records);
    Record::update_audio_field(records);
}

fn create_output_csv(records: &mut Vec<Record>, output_phonetics: Phonetics, input_file: &ApplicationFile) -> Result<(), Box<dyn Error>> {
    let file_name = input_file.get_file_name();
    csv_reader_writer::write_to_output_csv(records, file_name)?;

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    setup_application()?;

    let csv_file = select_input_csv_file();
    let mut csv_records = csv_reader_writer::get_csv_records(&csv_file)?;
    if csv_records.len() > 0 {
        let input_phonetics = phonetics_settings::infer_phonetics(&csv_records[0])?;
        let output_phonetics = select_output_phonetics_setting(&csv_records);

        Record::update_audio_file_name(&mut csv_records, input_phonetics)?;

        select_blacklist_audio_settings(&mut csv_records);
        download_audio_assets(&mut csv_records);
        create_output_csv(&mut csv_records, output_phonetics, &csv_file)?;
    } else {
        println!("No records were found from selected input csv file.");
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
