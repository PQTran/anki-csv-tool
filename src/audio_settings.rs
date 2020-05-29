use crate::csv_reader_writer::Record;
use crate::file_menu::SelectionOption;
use read_input::prelude::*;

fn print_record_characters(records: &Vec<Record>) {
    const CHUNK_SIZE: usize = 6;

    let options = records.iter().map(|record| {
        SelectionOption::new(record.get_id().clone(), record.get_character().clone())
    }).collect();

    SelectionOption::print_in_chunks(&options, CHUNK_SIZE);
}

fn get_blacklist_record_prompt(records: &Vec<Record>) -> String {
    const DEFAULT_PROMPT: &str = ">>> ";
    const BLACKLIST_PROMPT: &str = " >> ";
    let mut prompt = String::new();

    let blacklist_records = get_blacklist_from_records(records);
    for (index, record) in blacklist_records.iter().enumerate() {
        if index == 0 {
            prompt += &format!("[{}]", record.get_character());
        } else {
            prompt += &format!(", [{}]", record.get_character());
        }
    }

    if prompt.len() == 0 {
        prompt.push_str(DEFAULT_PROMPT);
    } else {
        prompt.push_str(BLACKLIST_PROMPT);
    }

    prompt
}

fn get_blacklist_from_records(records: &Vec<Record>) -> Vec<&Record> {
    let blacklist_records = records.iter().filter(|x| !x.get_download_audio()).collect();
    blacklist_records
}

fn get_mut_blacklist_from_records(records: &mut Vec<Record>) -> Vec<&mut Record> {
    let blacklist_records = records.iter_mut().filter(|x| !x.get_download_audio()).collect();
    blacklist_records
}

fn get_option_keys(records: &Vec<Record>) -> Vec<String> {
    let option_keys = records.iter().map(|x| x.get_id().to_string()).collect();
    option_keys
}

fn blacklist_contains(records: &Vec<Record>, id: &str) -> bool {
    let blacklist_records = get_blacklist_from_records(records);
    blacklist_records.iter().any(|x| x.get_id() == id)
}

// fn test(input_str: String) -> Option<&'static String> {
//     Some(&input_str)
// }

// fn get_mut_record_by_id(records: &Vec<Record>, id: &str) -> Option<&'static mut Record> {
//     let record = records.iter_mut().find(|x| x.get_id() == id);
//     record
// }

pub fn select_from_blacklist_audio_menu(csv_records: &mut Vec<Record>) {
    print_record_characters(&csv_records);

    println!("Blacklist: [Enter] to continue, [-] to pop last selection]");

    const ENTER_KEY: &str = "";
    const MINUS_KEY: &str = "-";
    let default_err_msg = "Your response was not recognized. Please try again!";
    loop {
        let mut options = get_option_keys(csv_records);
        options.push(ENTER_KEY.to_string());
        options.push(MINUS_KEY.to_string());

        let selected_id: String = input()
            .msg(get_blacklist_record_prompt(csv_records))
            .inside(options)
            .err(default_err_msg)
            .get();

        if ENTER_KEY.to_string() == selected_id {
            break;
        } else if MINUS_KEY.to_string() == selected_id {
            let mut blacklist_records = get_mut_blacklist_from_records(csv_records);
            if let Some(record) = blacklist_records.pop() {
                record.set_download_audio(true);
            }
        } else if blacklist_contains(csv_records, &selected_id) {
            println!("Your selection has been previously selected.");
        } else {
            if let Some(selected_record) = csv_records.iter_mut().find(|x| x.get_id() == &selected_id) {
                selected_record.set_download_audio(false);
            }
        }
    }
}
