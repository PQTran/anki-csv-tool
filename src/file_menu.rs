use crate::application_directory::ApplicationFile;
use read_input::prelude::*;

pub struct SelectionOption {
    key: String,
    value: String
}

impl SelectionOption {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }

    pub fn to_str(&self) -> String {
        format!("[{}] {}", self.key, self.value)
    }

    fn print_inline(options: &[SelectionOption]) {
        let mut msg = String::new();
        for option in options {
            if !msg.is_empty() {
                msg += ", ";
            }

            let option_str = option.to_str();
            msg += &option_str;
        }

        println!("{}", msg);
    }

    pub fn print_in_chunks(options: &Vec<SelectionOption>, chunk_size: usize) {
        for chunk in options.chunks(chunk_size) {
            SelectionOption::print_inline(chunk);
        }
    }
}

fn print_file_options(files: &Vec<ApplicationFile>) {
    for file in files {
        let file_key = file.get_key();
        let file_name = file.get_file_name();
        let option = SelectionOption::new(String::from(file_key), String::from(file_name));
        println!("{}", option.to_str());
    }
}

pub fn select_from_file_menu(files: &Vec<ApplicationFile>) -> &ApplicationFile {
    print_file_options(files);

    println!("Which csv file would you like to use?");

    let default_err_msg = "Your response was not recognized. Please try again!";
    let file_keys: Vec<String> = files.iter().map(|x| x.get_key().to_string()).collect();
    let selected_key: String = input()
        .msg(">> ")
        .inside(file_keys)
        .err(default_err_msg)
        .get();

    files.iter().find(|&file| file.get_key() == selected_key).unwrap()
}
