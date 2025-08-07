use std::fs::read_to_string;
use std::fs::File;
use std::io;
use std::fs;
use std::io::Write;
use std::time::Duration;
use directories::ProjectDirs;
use json::object;
use json::JsonValue;
use std::path::PathBuf;

use crate::word_tree::Word;

pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn write_json_to_file(json_to_write: JsonValue) -> Result<(), io::Error> {
    let path = get_stats_path();
    let mut file = File::create(path)?;
    let stringifyed_json = json::stringify(json_to_write);
    file.write_all(stringifyed_json.as_bytes())?;
    Ok(())
}

fn get_empty_json() -> JsonValue {
    let mut json = object! { all: { attempts: [] } };
    for letter in ALPHABET.chars() {
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

fn calc_accuracy(words: &Vec<&Word>) -> f64 {
    
    let length: usize = words.iter().map(|word| word.output.len()).sum();
    if length < 1 {
        return 0.0;
    }
    let count_correct_letters_in_word = 
        |word: &&Word| word.letters.iter().filter(|letter| letter.correct.unwrap_or(false) ).count();
    let total_correct_letters: usize = words.iter().map(|word| count_correct_letters_in_word(word) ).sum();
    let acc: f64 = (total_correct_letters as f64 / length as f64) * 100f64;
    acc
}

fn calc_wpm(words: &Vec<&Word>) -> f64 {

    let words: Vec<&&Word> = words
        .iter()
        .filter(
            |word| word.letters
                .iter()
                .all(
                    |letter| letter.correct.unwrap_or(false)
                )
        )
        .collect();

    let character_count: f64 = words.iter().map(|word| word.letters.len() as f64).sum();

    let all_words_durations = words.iter().map(|word| word.time);

    let mut total_duration = Duration::ZERO;

    for word_duration_opt in all_words_durations {
        if word_duration_opt.is_none() {
            continue;
        }
        let word_duration = word_duration_opt.unwrap();
        total_duration += word_duration;
    }

    if total_duration.is_zero() {
        return 0.0;
    }

    let wpm = (character_count / 5f64) / (total_duration.as_secs_f64() / 60f64);
    wpm
}


pub fn add_new_result(words: Vec<Word>) {

    let all_acc = calc_accuracy(&words.iter().collect());
    let all_wpm = calc_wpm(&words.iter().collect());

    let mut new_json = object! {
        all: { acc: all_acc, wpm: all_wpm }
    };

    for letter in ALPHABET.chars() {
        let words_with_letter: Vec<&Word> = words.iter().filter(|word| word.output.contains(letter)).collect();

        if words_with_letter.len() == 0 {
            continue;
        }
        let letter_acc = calc_accuracy(&words_with_letter);
        let letter_wpm = calc_wpm(&words_with_letter);

        new_json[letter.to_string()] = object! {
            acc: letter_acc,
            wpm: letter_wpm,
        };
    }
    let res = update_stats(new_json);
    if res.is_err() {
        println!("Failed to update json with this error: {:?}", res.err())
    }
}


#[allow(dead_code)]
fn update_stats(new_json: JsonValue) -> Result<(), io::Error> {
    let mut content = get_json_from_file()?;

    for entry in new_json.entries() {
        let letter = entry.0.to_string();
        let value = entry.1;

        content[&letter]["attempts"].push(object! {
            acc: value["acc"].clone(),
            wpm: value["wpm"].clone(),
        }).expect("There is something wrong with the json.");
    }
    write_json_to_file(content)?;
    Ok(())
}


pub fn show_stats() -> Result<(), io::Error> {

    let content = get_json_from_file()?;

    let mut letter_line = String::from("           ");
    let mut first_line = String::from("accuracy %:");
    let mut second_line = String::from("wpm:       ");

    for entry in content.entries() {
        let letter = entry.0;
        letter_line.push_str(format!("{letter:>7}").as_str());

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
            first_line = format!("{first_line}{accuracy:7.1}");
        }

        let wpm_opt = &last_attempt["wpm"].as_f32();
        if wpm_opt.is_none() {
            second_line.push_str("    N/A");
        } else {
            let wpm = wpm_opt.unwrap();
            second_line = format!("{second_line}{wpm:7.1}");
        }
    }

    println!("{}", letter_line);
    println!("{}", first_line);
    println!("{}", second_line);
    Ok(())
}


pub fn get_letter_data(letter: &String, is_wpm: bool) -> Vec<usize> {
    let content = get_json_from_file();
    if content.is_err() {
        println!("Got this error while reading statistics from file: {:?}", content.as_ref().err())
    }

    let content = content.unwrap();
    let attempts = &content[letter]["attempts"];
    let wpm_or_acc = if is_wpm { "wpm" } else { "acc" };

    let nums_in_string: Vec<String> = attempts
        .members()
        .map(|attempt| attempt[wpm_or_acc].to_string())
        .collect();
    let nums: Vec<usize> = nums_in_string
        .iter()
        .map(|string| string.parse::<f32>())
        .filter(|res| res.is_ok())
        .map(|ok_res| ok_res.unwrap() as usize)
        .collect();
    nums
}