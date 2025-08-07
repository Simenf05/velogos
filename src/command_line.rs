use std::{env, process};

use crate::{plot::{PlotData, PlotType}, statistics::{self, ALPHABET}};

#[derive(Debug)]
pub enum GameMode {
    HELP,
    ENDLESS,
    STATS,
    LESSON,
    PLOT,
} 

#[derive(Debug)]
pub struct GameOpts {
    pub mode: GameMode,
    pub file: Option<String>,
    pub plot_data: Option<PlotData>,
}

fn check_plot_type(plot_type: String) -> bool {
    match plot_type.as_str() {
        "all" => true,
        _ => ALPHABET.contains(&plot_type) && plot_type.len() == 1,
    }
}

pub fn show_help() {
    println!("velogos [--help|--stats|--endless] [--plot wpm|accuracy|sin|square all|letters...] [--file ...]")
}

fn parse_long_plot(args: &Vec<String>) -> Option<GameOpts> {

    let plot_data: Option<PlotData>;

    if !check_plot_type(args[3].clone()) {
        return None;
    }

    if args[3].as_str() == "all" {
        let plot_type = match args[2].as_str() {
            "wpm" => PlotType::AllWpm,
            "accuracy" => PlotType::AllAcc,
            _ => return None,
        };
        plot_data = Some(PlotData { plot_type, letter: None })
    }
    else {
        let letter = args[3].clone();
        let plot_type = match args[2].as_str() {
            "wpm" => PlotType::LetterWpm,
            "accuracy" => PlotType::LetterAcc,
            _ => return None,
        };
        plot_data = Some(PlotData { plot_type, letter: Some(letter) })
    }

    Some(GameOpts { mode: GameMode::PLOT, file: None, plot_data })
}


fn parse_three_or_more_options(args: &Vec<String>) -> Option<GameOpts> {
    if args.len() < 4 {
        return None;
    }

    let first_element = &args[1];

    let game_opts = match first_element.as_str() {
        "--plot" => {
            parse_long_plot(args)
        },
        _ => {
            let first_part = args.clone();
            let second_part = args.clone().split_off(1);

            let first_options = parse_one_option(&first_part);
            let second_options = parse_two_options(&second_part);

            if first_options.is_none() || second_options.is_none() {
                return None;
            }

            Some(GameOpts { mode: first_options.unwrap().mode, file: second_options.unwrap().file, plot_data: None })
        }
    };

    game_opts
}

fn parse_two_options(args: &Vec<String>) -> Option<GameOpts> {

    let first_opt = args.get(1);
    if first_opt.is_none() {
        return None;
    }
    let first = first_opt.unwrap();

    let mode: GameMode = match first.as_str() {
        "--plot" => GameMode::PLOT,
        "--file" => GameMode::LESSON,
        _ => { 
            show_help(); 
            process::exit(1) 
        },
    };

    let second_opt = args.get(2);
    if second_opt.is_none() {
        return None;
    }
    let second = second_opt.unwrap();

    if let GameMode::PLOT = mode {
        if second.as_str() == "sin" {
            return Some(GameOpts { mode, file: None, plot_data: Some(PlotData { plot_type: crate::plot::PlotType::Sin, letter: None })})
        }
        if second.as_str() == "square" {
            return Some(GameOpts { mode, file: None, plot_data: Some(PlotData { plot_type: crate::plot::PlotType::Square, letter: None })})
        }
        return None;
    }

    Some(GameOpts { mode, file: Some(second.clone()), plot_data: None })
}

fn parse_one_option(args: &Vec<String>) -> Option<GameOpts> {

    let first_opt = args.get(1);

    if first_opt.is_none() {
        return None;
    }
    let first = first_opt.unwrap();

    let opts = match first.as_str() {
        "--help" => Some(GameOpts { mode: GameMode::HELP, file: None, plot_data: None }),
        "--stats" => Some(GameOpts { mode: GameMode::STATS, file: None, plot_data: None }),
        "--endless" => Some(GameOpts { mode: GameMode::ENDLESS, file: None, plot_data: None }),
        _ => None,
    };

    opts
}

pub fn parse_command_line() -> GameOpts {
    let args: Vec<String> = env::args().collect();

    let opts = match args.len() - 1 {
        0 => Some(GameOpts { mode: GameMode::LESSON, file: None, plot_data: None }),
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
