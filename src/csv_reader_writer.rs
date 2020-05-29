use crate::application_directory::ApplicationFile;
use crate::application_directory::DataDirectory;
use crate::application_directory;
use crate::phonetics_settings::Phonetics;
use csv::ReaderBuilder;
use csv::Reader;
use csv::WriterBuilder;
use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

const TONES_PATTERN: &[char] = &['1', '2', '3', '4'];

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Record {
    #[serde(skip)]
    id: String,
    #[serde(default = "default_as_true", skip_serializing)]
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

    fn set_pronunciation(&mut self, val: String) {
        self.pronunciation = val;
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

fn get_writer(path: &PathBuf) -> Result<Writer<File>, Box<dyn Error>> {
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

fn split(pinyin_phrase: &str) -> Vec<String> {
    let mut pinyin_words = Vec::new();
    let mut start_index = 0;
    let mut done = false;

    while start_index < (pinyin_phrase.len() - 1) && !done {
        let mut word = String::new();
        let unsearched_phrase = String::from(&pinyin_phrase[start_index..]);
        if let Some(mut word_end) = unsearched_phrase.find(TONES_PATTERN) {
            word_end += 1; // inclusive gap
            word.push_str(&unsearched_phrase[0..word_end]);
            start_index += word_end;
        } else {
            word.push_str(&pinyin_phrase[start_index..]);
            done = true;
        }

        pinyin_words.push(word);
    }

    pinyin_words
}

pub fn update_record(record: &mut Record, phonetics_setting: Phonetics) {
    update_audio_field(record);
    update_pronunciation_field(record, phonetics_setting);
}

fn encode_to(pinyin_phrase: &str, phonetics_setting: Phonetics) -> String {
    let pinyin_words = split(pinyin_phrase);

    let encoded_words: Vec<String> = pinyin_words.iter().map(|word| {
        let mut encoded_val: String = Default::default();
        if phonetics_setting == Phonetics::Pinyin {
            if let Some(encoded_pinyin) = pinyin_zhuyin::encode_pinyin(word) {
                encoded_val.push_str(&encoded_pinyin);
            } else {
                // no tone for word
                encoded_val.push_str(word);
            }
        } else {
            let mut word = String::from(word);
            if !&word.contains(TONES_PATTERN) {
                word.push('5');
            }

            if let Some(encoded_zhuyin) = pinyin_zhuyin::encode_zhuyin(word) {
                encoded_val.push_str(&encoded_zhuyin);
            }
        }

        encoded_val
    }).collect();

    let encoded_phrase = encoded_words.join("");
    encoded_phrase
}

fn update_pronunciation_field(record: &mut Record, phonetics_setting: Phonetics) {
    let pronunciation = record.get_pronunciation();
    let encoded_pronunciation = encode_to(&pronunciation, phonetics_setting);
    // println!("updated field: {}", encoded_pronunciation);
    record.set_pronunciation(encoded_pronunciation);
}

fn update_audio_field(record: &mut Record) {
    let audio_str = format!("[sound:{}.mp3]", record.get_pronunciation());

    record.audio = String::from(audio_str);
}

fn write_to(records: &Vec<Record>, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut wtr = get_writer(&path)?;
    for record in records.iter() {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}

pub fn write_to_output_csv(records: &Vec<Record>, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut output_path = application_directory::get_data_directory_path(DataDirectory::OutputCsv);
    output_path.push(file_name);

    write_to(records, &output_path)?;
    Ok(())
}
