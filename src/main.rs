use crossterm::{cursor::{MoveLeft, MoveRight, MoveToColumn, SetCursorStyle}, event::{self, Event, KeyCode}, style::{Color, ResetColor, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, Clear}, ExecutableCommand};
use std::{cell::{Ref, RefCell}, io::{self, stdout, Write}, rc::Rc};

use crate::{command_line::{parse_command_line, show_help, GameMode, GameOpts}, statistics::{add_new_result, show_stats}, word_tree::{Node, Word}};

mod word_tree;
mod statistics;
mod command_line;

const LINE_LENGTH: u32 = 10;
const LINES_IN_LESSON: u16 = 3;

fn take_char() -> Option<KeyCode> {
    let res = event::poll(std::time::Duration::from_millis(100));

    if res.is_err() {
        return Option::None;
    }
    if res.unwrap() {
        
        let event = event::read();

        if event.is_err() {
            return Option::None;
        }

        if let Event::Key(key_event) = event.unwrap() {
            return Option::Some(key_event.code);
        }

    }
    Option::None
}

fn write_new_line(line: &String) -> Result<(), io::Error> {
    let mut stdout = stdout();
    stdout.execute(Clear(crossterm::terminal::ClearType::CurrentLine))?;
    stdout.execute(MoveToColumn(0))?;
    write!(stdout, "{}", line)?;
    stdout.execute(MoveToColumn(0))?;
    Ok(())
}


fn wrong_char(correct_char: char) -> Result<(), io::Error> {
    let mut stdout = stdout();
    stdout.execute(SetForegroundColor(Color::Red))?;
    write!(stdout, "{}",  correct_char)?;
    stdout.execute(ResetColor)?;
    stdout.execute(MoveLeft(1))?;
    stdout.flush()?;
    Ok(())
}

fn gen_line(root: Ref<'_, Node>, length: u32) -> Vec<Word> {
    let mut line = vec![];
    for _ in 0..length {
        line.push(root.gen_word_with_space());
    }
    line
}

fn gen_string_line(line: &Vec<Word>) -> String {
    let mut output_line = String::new();
    for word in line {
        output_line.push_str(&word.output);
    }
    output_line
}

fn typing_loop(root: Rc<RefCell<Node>>, opts: GameOpts) -> Result<(), io::Error> {

    if let GameMode::ENDLESS =  opts.mode {
        println!("Endless mode, click ESC to stop.")
    }
    let mut old_lines = vec!();

    let mut words = gen_line(root.borrow(), LINE_LENGTH);
    let mut line = gen_string_line(&words);

    let mut word_index = 0;
    let mut letter_index = 0;

    let mut completed_lines = 0u16;
    write_new_line(&line)?;

    loop {
        let current_word = &mut words[word_index];
        let code_opt = take_char();
        if code_opt.is_some() {
            let pressed_char = code_opt.unwrap();
            if pressed_char == KeyCode::Esc {
                break;
            }

            let correct_letter = &mut current_word.letters[letter_index];
            let correct_char = correct_letter.letter;

            if pressed_char == KeyCode::Char(correct_char) {
                stdout().execute(MoveRight(1))?;
                letter_index += 1;
                if correct_letter.correct.is_none() {
                    correct_letter.correct = Some(true);
                }
            }
            else {
                wrong_char(correct_char)?;
                correct_letter.correct = Some(false);
            }
        }

        if letter_index == current_word.letters.len() {
            letter_index = 0;
            word_index += 1;
        }

        if word_index == words.len() {
            completed_lines += 1;
            word_index = 0;

            for word in words {
                old_lines.push(word);
            }

            if let GameMode::LESSON = opts.mode {
                if completed_lines >= LINES_IN_LESSON {
                    add_new_result(old_lines);
                    break;
                }
            }

            words = gen_line(root.borrow(), LINE_LENGTH);
            line = gen_string_line(&words);
            write_new_line(&line)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), io::Error>{
    
    let opts = parse_command_line();
    
    if let GameMode::HELP = opts.mode {
        show_help();
        return Ok(());
    }
    if let GameMode::STATS = opts.mode {
        show_stats()?;
        return Ok(());
    }

    let mut file_name = String::from("1000-words");

    if opts.file.is_some() {
        file_name = opts.file.clone().unwrap();
    }

    let root = word_tree::Node::new(file_name.clone());

    if root.is_err() {
        let err_opt = root.err();

        if err_opt.is_some() {
            let err = err_opt.unwrap();
            println!("{}\nFile name: {file_name}", err.to_string());
        }
        return Ok(());
    }

    let mut stdout = stdout();

    enable_raw_mode()?;
    stdout.execute(SetCursorStyle::SteadyBar)?;

    typing_loop(root.unwrap(), opts)?;

    stdout.execute(SetCursorStyle::DefaultUserShape)?;
    stdout.execute(ResetColor)?;
    disable_raw_mode()?;
    println!();
    Ok(())
}
