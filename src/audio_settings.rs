use crate::models::Record;
use crate::models::SelectionOption;
use read_input::prelude::*;
use std::collections::HashMap;

fn _print_record_characters(options: &Vec<SelectionOption>) {
    const CHUNK_SIZE: usize = 6;
    SelectionOption::print_in_chunks(options, CHUNK_SIZE);
}

fn _get_blacklist_prompt(options: &Vec<SelectionOption>) -> String {
    const DEFAULT_PROMPT: &str = ">>> ";
    const BLACKLIST_PROMPT: &str = " >> ";
    let mut prompt = String::new();

    let iter = options.iter()
        .filter(|x| x.get_selected())
        .map(|x| x.get_value())
        .enumerate();

    for (index, character) in iter {
        if index == 0 {
            prompt += &format!("[{}]", character);
        } else {
            prompt += &format!(", [{}]", character);
        }
    }

    if prompt.len() == 0 {
        prompt.push_str(DEFAULT_PROMPT);
    } else {
        prompt.push_str(BLACKLIST_PROMPT);
    }

    prompt
}

fn _to_selection_options(records: &Vec<Record>) -> Vec<SelectionOption> {
    let options: Vec<SelectionOption> = records.iter().map(|x| {
        SelectionOption::new(x.get_id().to_string(), x.get_data().get_character().to_string())
    }).collect();

    options
}

pub fn select_from_blacklist_audio_menu(csv_records: &mut Vec<Record>) {
    let mut selection_options = _to_selection_options(csv_records);
    _print_record_characters(&selection_options);

    println!("Blacklist: [Enter] to continue, [-] to pop last selection]");

    const ENTER_KEY: &str = "";
    const MINUS_KEY: &str = "-";
    let default_err_msg = "Your response was not recognized. Please try again!";
    loop {
        let mut keys: Vec<String> = vec![ENTER_KEY.to_string(), MINUS_KEY.to_string()];
        let mut unselected_keys: Vec<String> = selection_options.iter()
            .filter(|x| !x.get_selected())
            .map(|x| x.get_key().to_string()).collect();
        keys.append(&mut unselected_keys);

        let selected_key: String = input()
            .msg(_get_blacklist_prompt(&selection_options))
            .inside(keys)
            .err(default_err_msg)
            .get();

        match selected_key.as_str() {
            ENTER_KEY => break,
            MINUS_KEY => {
                // !!! learning point: understand mutable pointer
                let mut option = selection_options.iter_mut().last().unwrap();
                option.set_selected(false);
            },
            _ => {
                let mut option = selection_options.iter_mut().find(|x| x.get_key() == selected_key).unwrap();
                option.set_selected(true);
            }
        }
    }

    _update_csv_records_using_selection_options(csv_records, selection_options);
}

fn _update_csv_records_using_selection_options(records: &mut Vec<Record>, options: Vec<SelectionOption>) {
    let option_map: HashMap<String, SelectionOption> = options.into_iter().map(|x| {
        (x.get_key().to_string(), x)
    }).collect();

    for record in records.iter_mut() {
        let option = option_map.get(record.get_id()).unwrap();

        record.set_download_audio(!option.get_selected());
    }
}
