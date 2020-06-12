use crate::models::{ApplicationFile, DataDirectory};
use directories::BaseDirs;
use directories::ProjectDirs;
use std::path::PathBuf;
use std::path::Path;
use std::error::Error;
use std::io;
use std::fs;

pub fn get_anki_audio_path() -> PathBuf {
    let base_dirs = BaseDirs::new().unwrap();
    let mut audio_path = base_dirs.data_local_dir().to_path_buf();
    audio_path.push("Anki2");
    audio_path.push("pqtran");
    audio_path.push("collection.media");
    audio_path
}

pub fn get_data_directory_path(dir: DataDirectory) -> PathBuf {
    use DataDirectory::*;

    let proj_dirs = ProjectDirs::from("", "", "Anki-Csv-Tool").expect("Unable to retrieve app directory.");
    let base_dir = proj_dirs.data_dir().to_str().unwrap();

    let mut dir_buf = PathBuf::from(base_dir);
    match dir {
        Base => {},
        Input => {
            dir_buf.push("input");
        },
        InputCsv => {
            dir_buf.push("input");
            dir_buf.push("csv");
        },
        Output => {
            dir_buf.push("output");
        },
        OutputCsv => {
            dir_buf.push("output");
            dir_buf.push("csv");
        },
        OutputAudio => {
            dir_buf.push("output");
            dir_buf.push("audio");
        }
    };

    dir_buf
}

fn _create_directory(path: &Path) -> io::Result<()> {
    if !path.is_dir() {
        fs::create_dir(path)?;
        println!("Created dir: {}", path.to_str().unwrap());
    }

    Ok(())
}

pub fn create_default_data_directories() -> Result<(), Box<dyn Error>> {
    use DataDirectory::*;
    let dirs = [Base, Input, InputCsv, Output, OutputCsv, OutputAudio];
    for dir in dirs.iter() {
        _create_directory(get_data_directory_path(*dir).as_path())?;
    }

    Ok(())
}

fn _get_initial_key() -> String {
    "1".to_string()
}

fn _get_next_key(s: &str) -> String {
    let mut num: u8 = s.parse().unwrap();
    num += 1;
    num.to_string()
}

// has 2 logic, one for path and one for key
pub fn get_input_csv_files() -> Vec<ApplicationFile> {
    const CSV_EXTENSION: &str = "csv";
    let mut csv_files = Vec::new();
    let mut key = _get_initial_key();


    let input_csv_path = get_data_directory_path(DataDirectory::InputCsv);
    for entry in fs::read_dir(&input_csv_path).expect("Unable to read input csv directory.") {
        if let Ok(entry) = entry {
            let path = entry.path();
            let extension = path.extension().unwrap_or_default();
            if let Some(extension_str) = extension.to_str() {
                if extension_str.to_lowercase() == CSV_EXTENSION {
                    let file = ApplicationFile::new(key.clone(), path);
                    csv_files.push(file);

                    key = _get_next_key(&key);
                }
            }
        }
    }

    csv_files
}
