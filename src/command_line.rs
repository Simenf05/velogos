use std::{env, process};

use crate::statistics;

pub enum GameMode {
    HELP,
    ENDLESS,
    STATS,
    LESSON,
} 

pub struct GameOpts {
    pub mode: GameMode,
    file: Option<String>,
}

pub fn show_help() {
    println!("typinghelp [--help|--stats|--endless] [--file ...]")
}


fn parse_three_or_more_options(args: &Vec<String>) -> Option<GameOpts> {
    let first_part = args.clone();
    let second_part = args.clone().split_off(1);

    let first_options = parse_one_option(&first_part);
    let second_options = parse_two_options(&second_part);

    if first_options.is_none() || second_options.is_none() {
        return None;
    }
    Some(GameOpts { mode: first_options.unwrap().mode, file: second_options.unwrap().file })
}

fn parse_two_options(args: &Vec<String>) -> Option<GameOpts> {

    let first_opt = args.get(1);
    if first_opt.is_none() {
        return None;
    }
    let first = first_opt.unwrap();

    if first.as_str() != "--file" {
        show_help();
        process::exit(1);
    }

    let second_opt = args.get(2);
    if second_opt.is_none() {
        return None;
    }
    let second = second_opt.unwrap();
    
    Some(GameOpts { mode: GameMode::LESSON, file: Some(second.clone()) })
}

fn parse_one_option(args: &Vec<String>) -> Option<GameOpts> {

    let first_opt = args.get(1);

    if first_opt.is_none() {
        return None;
    }
    let first = first_opt.unwrap();

    let opts = match first.as_str() {
        "--help" => Some(GameOpts { mode: GameMode::HELP, file: None }),
        "--stats" => Some(GameOpts { mode: GameMode::STATS, file: None }),
        "--endless" => Some(GameOpts { mode: GameMode::ENDLESS, file: None }),
        _ => None,
    };

    opts
}

pub fn parse_command_line() -> GameOpts {
    let args: Vec<String> = env::args().collect();

    let opts = match args.len() - 1 {
        0 => Some(GameOpts { mode: GameMode::LESSON, file: None }),
        1 => parse_one_option(&args),
        2 => parse_two_options(&args),
        _ => parse_three_or_more_options(&args),
    };

    if opts.is_none() {
        show_help();
        process::exit(1);
    }

    let opts = opts.unwrap();

    if let GameMode::HELP = opts.mode {
        show_help();
        process::exit(0);
    }
    if let GameMode::STATS = opts.mode {
        let res = statistics::show_stats();
        if res.is_err() {
            println!("{:?}", res.err());
            process::exit(1);
        }
        process::exit(0);
    }

    opts
}
