use directories::ProjectDirs;
use std::path::PathBuf;
use std::path::Path;
use std::error::Error;
use std::io;
use std::fs;

pub enum DataDirectory {
    Base,
    Input,
    InputCsv,
    Output,
    OutputCsv,
    OutputAudio
}

#[derive(Debug, PartialEq, Clone)]
pub struct ApplicationFile {
    key: String,
    file_path: PathBuf
}

impl ApplicationFile {
    fn new(key: String, file_path: PathBuf) -> Self {
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

pub fn get_data_directory_path(dir: &DataDirectory) -> PathBuf {
    use DataDirectory::*;

    let proj_dirs = ProjectDirs::from("", "", "Anki-Csv-Tool").expect("Unable to retrieve app directory.");
    let base_dir = proj_dirs.data_dir().to_str().unwrap();

    let mut dir_buf = PathBuf::from(base_dir);
    match &*dir {
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

fn create_directory(path: &Path) -> io::Result<()> {
    if ! path.is_dir() {
        fs::create_dir(path)?;
        println!("Created dir: {}", path.to_str().unwrap());
    }

    Ok(())
}

pub fn create_default_data_directories() -> Result<(), Box<dyn Error>> {
    use DataDirectory::*;
    let dirs = [Base, Input, InputCsv, Output, OutputCsv, OutputAudio];
    for dir in dirs.iter() {
        create_directory(get_data_directory_path(dir).as_path())?;
    }

    Ok(())
}

pub fn get_input_csv_files() -> Vec<ApplicationFile> {
    const CSV_EXTENSION: &str = "csv";
    let mut csv_files = Vec::new();
    let mut file_index = 1;

    let input_csv_path = get_data_directory_path(&DataDirectory::InputCsv);
    for read_dir in fs::read_dir(&input_csv_path).expect("Unable to read input csv directory.") {
        if let Ok(dir_entry) = read_dir {
            let path = dir_entry.path();
            if let Some(extension) = path.as_path().extension() {
                let extension = extension.to_str().unwrap();
                if extension.to_lowercase() == CSV_EXTENSION {
                    let file = ApplicationFile::new(file_index.to_string(), path);
                    csv_files.push(file);

                    file_index += 1;
                }
            }
        }
    }

    csv_files
}
