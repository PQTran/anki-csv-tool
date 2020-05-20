extern crate directories;

use directories::ProjectDirs;
use std::path::PathBuf;
use std::path::Path;
use std::io;
use std::fs;

enum DataDirectory {
    Base,
    Input,
    InputCsv,
    Output,
    OutputCsv,
    OutputAudio
}

fn get_data_directory_path(dir: &DataDirectory) -> PathBuf {
    use DataDirectory::*;
    let mut dir_buf = PathBuf::new();

    if let Some(proj_dirs) = ProjectDirs::from("", "", "Anki-Csv-Tool") {
        let base_dir = proj_dirs.data_dir().to_str().unwrap();

        dir_buf.push(base_dir);
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
    }

    dir_buf
}

fn create_directory(path: &Path) -> io::Result<()> {
    println!("{}", path.to_str().unwrap());

    if ! path.is_dir() {
        fs::create_dir(path)?;
    }

    Ok(())
}

pub fn create_default_data_directories() {
    use DataDirectory::*;
    let dirs = [Base, Input, InputCsv, Output, OutputCsv, OutputAudio];
    for dir in dirs.iter() {
        if let Err(e) = create_directory(get_data_directory_path(dir).as_path()) {
            println!("{}", "An error occured in creating default data directories. Please inspect.");
        }
    }
}

pub fn get_input_csv_files() -> Vec<PathBuf> {
    let mut csv_files = Vec::new();

    let input_csv_path = get_data_directory_path(&DataDirectory::InputCsv);
    for entry in fs::read_dir(&input_csv_path).expect("Unable to read input csv directory.") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.as_path().extension().unwrap() == "csv"  {
                println!("{:?}", path);
                csv_files.push(path);
            }
        }
    }

    csv_files
}
