use crate::application_directory::ApplicationFile;
use crate::phonetics_settings::Phonetics;
use csv::ReaderBuilder;
use csv::Reader;
use csv::WriterBuilder;
use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::env;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Record {
    #[serde(skip)]
    id: String,
    #[serde(default = "default_as_true")]
    download_audio: bool,
    character: String,
    pronunciation: String,
    definition: String,
    #[serde(skip_deserializing)]
    audio: String,
    notes: String,
    tags: String
}

impl Record {
    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_download_audio(&self) -> bool {
        self.download_audio
    }

    pub fn get_character(&self) -> &String {
        &self.character
    }

    pub fn get_pronunciation(&self) -> &String {
        &self.pronunciation
    }

    fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_download_audio(&mut self, val: bool) {
        self.download_audio = val;
    }

    pub fn print_preview(&self) {
        println!("character: {}, pronunciation: {}",
                 self.get_character(),
                 self.get_pronunciation());
    }
}

fn default_as_true() -> bool {
    true
}

pub fn get_reader(file: &ApplicationFile) -> Result<Reader<File>, Box<dyn Error>> {
    let rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file.get_file_path())?;

    Ok(rdr)
}

pub fn get_writer(path: &PathBuf) -> Result<Writer<File>, Box<dyn Error>> {
    let wtr = WriterBuilder::new()
        .has_headers(false)
        .from_path(path)?;

    Ok(wtr)
}

pub fn get_csv_records(file: &ApplicationFile) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut csv_records = Vec::new();
    let mut rdr = get_reader(file)?;

    for (index, result) in rdr.deserialize().enumerate() {
        let id = index + 1;
        let mut record: Record = result?;
        record.set_id(id.to_string());

        csv_records.push(record);
    }

    Ok(csv_records)
}

// currently use google api
// consider looking for other audio sources
// consider taiwanese audio sources as well
// consider finding api using phonetics-based rather than character based
// use tts_urls crate
fn download_audio_assets(file: &ApplicationFile, phonetics: &Phonetics) -> Result<(), Box<dyn Error>> {
    let mut rdr = get_reader(file)?;
    println!("download audio assets");
    Ok(())
}

fn format_csv(file: &ApplicationFile) -> Result<PathBuf, Box<dyn Error>> {
    let mut temp_file_path = env::temp_dir();
    temp_file_path.push(file.get_file_name());
    let mut wtr = get_writer(&temp_file_path)?;

    let mut rdr = get_reader(file)?;
    for result in rdr.deserialize() {
        let mut record: Record = result?;
        update_record(&mut record);
        wtr.serialize(record)?;
    }


    Ok(temp_file_path)
}

fn update_record(record: &mut Record) {
    update_pronunciation_field(record);
    update_audio_field(record);
}

fn update_pronunciation_field(record: &mut Record) {

}

fn update_audio_field(record: &mut Record) {
    record.audio = String::from("[sound:{}.mp3]");
}

fn copy_file_to(file: &File, path: &PathBuf) {

}

pub fn test_run(file: ApplicationFile) {
    // download_audio_assets(&file, &phonetics_setting);
    // let temp_file = format_csv(&file);

}

