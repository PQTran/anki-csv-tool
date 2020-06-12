use crate::models::{SelectionOption, ApplicationFile};
use read_input::prelude::*;

fn _print_file_options(files: &Vec<ApplicationFile>) {
    for file in files {
        let file_key = file.get_key();
        let file_name = file.get_file_name();
        let option = SelectionOption::new(String::from(file_key), String::from(file_name));
        println!("{}", option.to_str());
    }
}

pub fn select_from_file_menu(files: &Vec<ApplicationFile>) -> &ApplicationFile {
    _print_file_options(files);

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
