use crate::models::{ApplicationFile, DataDirectory, Record, RecordData};
use crate::application_directory;
use crate::phonetics_settings::Phonetics;
use crate::phonetics_settings;
use crate::audio_asset_manager;
use csv::ReaderBuilder;
use csv::Reader;
use csv::WriterBuilder;
use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

fn get_reader(file: &ApplicationFile, headers: bool) -> Result<Reader<File>, Box<dyn Error>> {
    let rdr = ReaderBuilder::new()
        .has_headers(headers)
        .from_path(file.get_file_path())?;

    Ok(rdr)
}

fn get_writer(path: &PathBuf, headers: bool) -> Result<Writer<File>, Box<dyn Error>> {
    let wtr = WriterBuilder::new()
        .has_headers(headers)
        .from_path(path)?;

    Ok(wtr)
}

pub fn get_csv_records(file: &ApplicationFile) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut csv_records = Vec::new();
    let mut rdr = get_reader(file, true)?;

    for (index, result) in rdr.deserialize().enumerate() {
        let id = index + 1;
        let data: RecordData = result?;
        let record = Record::new(id.to_string(), data);

        csv_records.push(record);
    }

    Ok(csv_records)
}

fn _write_to(records: &Vec<Record>, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut wtr = get_writer(&path, false)?;
    for record_data in records.iter().map(|x| x.get_data()) {
        wtr.serialize(record_data)?;
    }

    wtr.flush()?;
    Ok(())
}

pub fn write_to_output_csv(records: &Vec<Record>, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut output_path = application_directory::get_data_directory_path(DataDirectory::OutputCsv);
    output_path.push(file_name);

    _write_to(records, &output_path)?;
    Ok(())
}
