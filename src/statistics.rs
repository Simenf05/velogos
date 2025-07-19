use std::fs::read_to_string;
use std::fs::File;
use std::io;
use std::fs;
use std::io::Write;
use directories::ProjectDirs;
use json::object;
use json::JsonValue;
use std::path::PathBuf;


fn write_json_to_file(json_to_write: JsonValue) -> Result<(), io::Error> {
    let path = get_stats_path();
    let mut file = File::create(path)?;
    let stringifyed_json = json::stringify(json_to_write);
    file.write_all(stringifyed_json.as_bytes())?;
    Ok(())
}


fn get_empty_json() -> JsonValue {
    let mut json = object! {};
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    for letter in alphabet.chars() {
        let inner_json = object! {
            attempts: [],
        };
        json[letter.to_string()] = inner_json;
    }
    json
}



fn get_stats_path() -> PathBuf {
    if let Some(dir) = ProjectDirs::from("org", "fritsvold", "velogos") {

        let dir_path: PathBuf = dir.data_dir().to_path_buf();
        let create_res = fs::create_dir_all(&dir_path);

        if create_res.is_err() {
            println!("Failed to make project directory.");
            std::process::exit(1);
        }

        let file_path = dir_path.join("letter_data.json");
        let new_file = File::create_new(&file_path);

        if new_file.is_ok() {
            let empty_json = get_empty_json();
            let res = write_json_to_file(empty_json);

            if res.is_err() {
                println!("Could not write to file.");
                std::process::exit(1);
            }
        }
        
        return file_path;

    }
    else {
        println!("Could not find project directory path.");
        std::process::exit(1);
    }
}

fn read_file_as_string() -> Result<String, io::Error> {
    let path = get_stats_path();
    let content = read_to_string(path)?;
    Ok(content)
}
fn get_json_from_file() -> Result<JsonValue, io::Error> {
    let string_content = read_file_as_string()?;
    let content = json::parse(&string_content).expect("Failed to parse json.");
    Ok(content)
}


pub fn _update_stats() {



    let _ = write_json_to_file(get_empty_json());
}


pub fn show_stats() -> Result<(), io::Error> {
    let content = get_json_from_file()?;

    let mut letter_line = String::from("           ");
    let mut first_line = String::from("accuracy %:");
    let mut second_line = String::from("wpm:       ");

    for entry in content.entries() {
        let letter = entry.0;
        letter_line.push_str("      ");
        letter_line.push_str(letter);

        let attempts = &entry.1["attempts"];
        let attempts_count = attempts.len();

        if attempts_count == 0 {
            first_line.push_str("    N/A");
            second_line.push_str("    N/A");
            continue;
        }

        let last_attempt = &attempts[attempts_count - 1];

        let accuracy_opt = &last_attempt["acc"].as_f32();
        if accuracy_opt.is_none() {
            first_line.push_str("    N/A");
        } else {
            let accuracy = accuracy_opt.unwrap();
            first_line = format!("{first_line}{accuracy:7}");
        }

        let wpm_opt = &last_attempt["wpm"].as_f32();
        if wpm_opt.is_none() {
            second_line.push_str("    N/A");
        } else {
            let wpm = wpm_opt.unwrap();
            second_line = format!("{second_line}{wpm:7}");
        }
    }

    println!("{}", letter_line);
    println!("{}", first_line);
    println!("{}", second_line);
    Ok(())
}
