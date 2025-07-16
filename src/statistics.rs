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
            accuracy: null,
            wpm: null,
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
        let _ = File::create_new(&file_path);
        
        return file_path;

    }
    else {
        println!("Could not find project directory path.");
        std::process::exit(1);
    }
}

fn get_json_from_file() -> Result<String, io::Error> {
    let path = get_stats_path();
    let content = read_to_string(path)?;
    Ok(content)
}


pub fn update_stats() {
    println!("To be implemented...");
    let _ = write_json_to_file(object! {});
}


pub fn show_stats() -> Result<(), io::Error> {

    let path = get_stats_path();
    let content = get_json_from_file()?;

    let json = get_empty_json();

    println!("{:?}", json);

    println!("{}", json::stringify(json));


    println!("{}", path.display());
    println!("{}", content);
    
    Ok(())
}
