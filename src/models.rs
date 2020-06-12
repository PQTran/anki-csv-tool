use std::path::PathBuf;
use std::error::Error;

#[derive(Clone, Copy)]
pub enum DataDirectory {
    Base,
    Input,
    InputCsv,
    Output,
    OutputCsv,
    OutputAudio
}

#[derive(Debug)]
pub struct Record {
    id: String,
    audio_file_name: String,
    download_audio: bool,
    data: RecordData
}

impl Record {
    pub fn new(id: String, data: RecordData) -> Self {
        Record { id, data, audio_file_name: String::new(), download_audio: true }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_audio_file_name(&self) -> &str {
        &self.audio_file_name
    }

    pub fn get_download_audio(&self) -> bool {
        self.download_audio
    }

    pub fn get_data(&self) -> &RecordData {
        &self.data
    }

    pub fn get_mut_data(&mut self) -> &mut RecordData {
        &mut self.data
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_audio_file_name(&mut self, file_name: &str) {
        self.audio_file_name.clear();
        self.audio_file_name.push_str(file_name);
    }

    pub fn set_download_audio(&mut self, val: bool) {
        self.download_audio = val;
    }

    pub fn print_preview(&self) {
        println!("character: {}, pronunciation: {}",
                 self.data.get_character(),
                 self.data.get_pronunciation());
    }

    pub fn update_audio_file_name(records: &mut Vec<Record>, input_phonetics: crate::phonetics_settings::Phonetics) -> Result<(), Box<dyn Error>> {
        let get_mp3_name = |file_name: &str| format!("{}.mp3", file_name);

        for record in records.iter_mut() {
            let mut file_name = String::new();

            let pronunciation = record.get_data().get_pronunciation();
            for word in zhuyin::split(pronunciation).iter() {
                let pinyin = zhuyin::to_pinyin(word)?;
                file_name.push_str(&pinyin);
            }

            file_name = get_mp3_name(&file_name);
            record.set_audio_file_name(&file_name);
        }

        Ok(())
    }

    pub fn update_audio_field(records: &mut Vec<Record>) {
        let format_audio_str = |file_name: &str| format!("[sound:{}]", file_name);

        for record in records.iter_mut() {
            let audio_str = format_audio_str(record.get_audio_file_name());

            let mut record_data = record.get_mut_data();
            record_data.set_audio(audio_str);
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecordData {
    character: String,
    pronunciation: String,
    definition: String,
    #[serde(skip_deserializing)]
    audio: String,
    notes: String,
    tags: String
}

impl RecordData {
    pub fn get_character(&self) -> &str {
        &self.character
    }

    pub fn get_pronunciation(&self) -> &str {
        &self.pronunciation
    }

    pub fn set_pronunciation(&mut self, val: String) {
        self.pronunciation = val;
    }

    pub fn set_audio(&mut self, val: String) {
        self.audio = val;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ApplicationFile {
    key: String,
    file_path: PathBuf
}

impl ApplicationFile {
    pub fn new(key: String, file_path: PathBuf) -> Self {
        Self { key, file_path }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_file_name(&self) -> &str {
        let path = self.file_path.as_path();
        let file_name = path.file_name().unwrap();
        file_name.to_str().unwrap()
    }

    pub fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

pub struct SelectionOption {
    key: String,
    value: String,
    selected: bool
}

impl SelectionOption {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value, selected: false }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn get_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, val: bool) {
        self.selected = val;
    }

    pub fn to_str(&self) -> String {
        format!("[{}] {}", self.key, self.value)
    }

    pub fn format_inline(options: &[SelectionOption]) -> String {
        let mut msg = String::new();
        for option in options {
            if !msg.is_empty() {
                msg += ", ";
            }

            let option_str = option.to_str();
            msg += &option_str;
        }

        format!("{}", msg)
    }

    fn print_inline(options: &[SelectionOption]) {
        let msg = SelectionOption::format_inline(options);
        println!("{}", msg);
    }

    pub fn print_in_chunks(options: &Vec<SelectionOption>, chunk_size: usize) {
        for chunk in options.chunks(chunk_size) {
            SelectionOption::print_inline(chunk);
        }
    }
}
