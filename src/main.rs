use crossterm::{cursor::{MoveLeft, MoveRight, MoveToColumn, SetCursorStyle}, event::{self, Event, KeyCode}, style::{Color, ResetColor, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, Clear}, ExecutableCommand};
use std::{cell::{Ref, RefCell}, env, io::{self, stdout, Write}, rc::Rc};

use crate::{command_line::{parse_command_line, show_help, GameMode}, statistics::show_stats, word_tree::Node};

mod word_tree;
mod statistics;
mod command_line;

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

fn gen_line(root: Ref<'_, Node>, length: u32) -> String {
    let mut line = root.gen_word();

    for _ in 1..length {
        line.push(' ');
        line.push_str(&root.gen_word());
    }

    line
}

fn typing_loop(root: Rc<RefCell<Node>>) -> Result<(), io::Error> {

    let mut line = gen_line(root.borrow(), 10);
    let mut completed_chars = 0;
    write_new_line(&line)?;

    loop {
        let code_opt = take_char();
        if code_opt.is_some() {
            let pressed_char = code_opt.unwrap();
            if pressed_char == KeyCode::Esc {
                break;
            }

            let correct_char_opt = line.chars().nth(completed_chars);
            if correct_char_opt.is_none() {
                continue;
            }
            let correct_char = correct_char_opt.unwrap();

            if pressed_char == KeyCode::Char(correct_char) {
                stdout().execute(MoveRight(1))?;
                completed_chars += 1;
            }
            else {
                wrong_char(correct_char)?;
            }
        }

        if completed_chars == line.len() {
            line = gen_line(root.borrow(), 10);
            completed_chars = 0;
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


    let root = word_tree::Node::new()?;
    let mut stdout = stdout();

    enable_raw_mode()?;
    stdout.execute(SetCursorStyle::SteadyBar)?;

    typing_loop(root)?;

    stdout.execute(SetCursorStyle::DefaultUserShape)?;
    stdout.execute(ResetColor)?;
    disable_raw_mode()?;
    println!();
    Ok(())
}
