use std::{
    fmt::{self},
    usize, vec,
};

use crate::statistics::get_letter_data;

#[derive(Debug)]
pub struct PlotData {
    pub plot_type: PlotType,
    pub letter: Option<String>,
}

#[derive(Debug)]
pub enum PlotType {
    LetterWpm,
    AllWpm,
    LetterAcc,
    AllAcc,
    Sin,
    Square,
}

struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }
}

struct Plot {
    grid: Vec<String>,
    width: usize,
    height: usize,
}

impl Plot {
    fn new(width: usize, height: usize) -> Plot {
        let mut grid = vec![];

        for _ in 0..height {
            grid.push(format!("{:width$}", " "));
        }

        Plot {
            grid,
            width,
            height,
        }
    }

    fn translate_coords(coord: Coord, width: usize, height: usize) -> Result<Coord, String> {
        if coord.x > width {
            return Err(format!("The x coordinate is too big, got: {}", coord.x));
        }
        if coord.y > height {
            return Err(format!("The y coordinate is too big, got: {}", coord.y));
        }

        let y = height - 1 - coord.y;
        Ok(Coord { x: coord.x, y: y })
    }

    fn place_star(&mut self, coord: Coord) -> Result<(), String> {
        let translated_coord = Plot::translate_coords(coord, self.width, self.height)?;

        let x = translated_coord.x;
        let y = translated_coord.y;

        let line_opt = self.grid.get_mut(y);

        if line_opt.is_none() {
            return Err(String::from("There is something wrong with the line."));
        }

        let line = line_opt.unwrap();

        line.replace_range(x..x + 1, "*");

        Ok(())
    }

    fn make_diagonal(&mut self, start_coord: Coord, end_coord: Coord) -> Result<(), String> {
        let dx: isize = end_coord.x as isize - start_coord.x as isize;
        let mut dy: isize = end_coord.y as isize - start_coord.y as isize;
        let mut reflect_y = false;
        if dy < 0 {
            reflect_y = true;
            dy = -dy;
        }

        let mut place_with_reflect_y = |coord: Coord| -> Result<(), String> {
            if reflect_y {
                self.place_star(Coord::new(
                    coord.x,
                    start_coord.y - (coord.y - start_coord.y),
                ))?;
            } else {
                self.place_star(Coord::new(coord.x, coord.y))?;
            }
            Ok(())
        };

        if dy <= dx {
            let mut d: isize = dy - (dx / 2);
            let mut x: usize = start_coord.x;
            let mut y: usize = start_coord.y;

            place_with_reflect_y(Coord::new(x, y))?;
            while x < end_coord.x {
                x += 1;
                if d < 0 {
                    d += dy;
                } else {
                    d = d + dy - dx;
                    y += 1;
                }

                place_with_reflect_y(Coord::new(x, y))?;
            }
        } else if dx <= dy {
            let mut d: isize = dx - (dy / 2);
            let mut x: usize = start_coord.x;
            let mut y: usize = start_coord.y;

            place_with_reflect_y(Coord::new(x, y))?;
            let condition = |y: usize| {
                if reflect_y {
                    start_coord.y - (y - start_coord.y) > end_coord.y
                } else {
                    y < end_coord.y
                }
            };
            while condition(y) {
                y += 1;

                if d < 0 {
                    d += dx;
                } else {
                    d = d + dx - dy;
                    x += 1;
                }

                place_with_reflect_y(Coord::new(x, y))?;
            }
        }

        Ok(())
    }
}

fn insert_spaces(line: &mut String) -> String {
    let mut new_line = String::new();
    let mut counter = 0;
    for c in line.chars() {
        new_line.push(c);
        new_line.push(' ');
        if counter % 3 == 0 {
            new_line.push(' ');
        }
        counter += 1;
    }
    new_line
}

impl fmt::Display for Plot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut width = self.width * 2;
        width += self.width / 3;
        if self.width % 3 != 0 {
            width += 1;
        }

        let first_and_last_line = format!("+{:-<width$}+", "");
        let mut grid = self.grid.clone();
        grid = grid
            .iter_mut()
            .map(|line| insert_spaces(line))
            .map(|line| format!("|{}|", line))
            .collect();
        let output = grid.join("\n");
        write!(
            f,
            "{first_and_last_line}\n{}\n{first_and_last_line}\n",
            output
        )
    }
}

fn find_scaler(biggest_num: u16, size: u16) -> f32 {
    let mut scaler = 1.0;
    let biggest_num_float = biggest_num as f32;
    while biggest_num_float * scaler >= size as f32 {
        scaler -= 0.01
    }
    scaler
}

fn equal_remove(
    arr: &mut [usize],
    start: usize,
    end: usize,
    mut amount: u16,
    is_left: bool,
) -> u16 {
    let length = end - start;

    let divide_by_two_up = |x: f32| (x / 2f32).ceil();
    let divide_by_two_down = |x: f32| (x / 2f32).floor();
    let calc_sep = |length: usize| -> usize {
        if length % 2 == 0 {
            (if is_left {
                divide_by_two_up(length as f32)
            } else {
                divide_by_two_down(length as f32)
            }) as usize
        } else {
            divide_by_two_down(length as f32) as usize
        }
    };

    let sep = calc_sep(length) + start;
    let mid = &mut arr[sep];

    let mut removed = 0u16;

    if amount % 2 != 0 {
        *mid = usize::MAX;
        amount -= 1;
        removed += 1;
    }

    if length == amount as usize {
        for i in 0..amount as usize {
            arr[start + i] = usize::MAX;
        }
        return amount;
    }

    if is_left {
        if length >= 2 {
            removed += equal_remove(arr, start, sep, amount / 2, false);
        }

        if length >= 3 {
            removed += equal_remove(arr, sep + 1, end, amount / 2, true);
        }
    } else {
        if length >= 2 {
            removed += equal_remove(arr, start, sep, amount / 2, true);
        }

        if length >= 3 {
            removed += equal_remove(arr, sep + 1, end, amount / 2, false);
        }
    }

    return removed;
}

#[allow(dead_code)]
fn test_equal_remove() {
    for i in 0..200u16 {
        for j in 0..i {
            let mut vec = vec![];

            for k in 0..i {
                vec.push(k as usize);
            }

            let arr = vec.as_mut_slice();
            let len = arr.len();

            let amount = j;
            let removed = equal_remove(arr, 0, len, amount, false);
            let mut new_nums = vec![];

            for el in arr {
                if *el == usize::MAX {
                    continue;
                }
                new_nums.push(el);
            }

            assert_eq!(removed, amount);
            assert_eq!(new_nums.len(), len - amount as usize);
        }
    }
}

#[allow(dead_code)]
pub fn get_square() -> String {
    let f = |x: f32| x * x;

    let mut nums = vec![];
    let function = f;

    for i in -22..22 {
        nums.push(function(i as f32) as usize);
        nums.push(function(i as f32 + 0.2) as usize);
        nums.push(function(i as f32 + 0.4) as usize);
        nums.push(function(i as f32 + 0.6) as usize);
        nums.push(function(i as f32 + 0.8) as usize);
    }

    get_plot(nums)
}

#[allow(dead_code)]
pub fn get_tan() -> String {
    let tan = |x: f32| f32::tan(x) * 40f32 + 500f32;

    let mut nums = vec![];
    let function = tan;

    for i in 0..15 {
        nums.push(function(i as f32));
        nums.push(function(i as f32 + 0.2));
        nums.push(function(i as f32 + 0.4));
        nums.push(function(i as f32 + 0.6));
        nums.push(function(i as f32 + 0.8));
    }
    get_plot(nums.iter().map(|a| *a as usize).collect())
}

pub fn get_sin() -> String {
    let sin = |x: f32| f32::sin(x) * 8f32 + 22f32;

    let mut nums = vec![];
    let function = sin;

    for i in 0..28 {
        nums.push(function(i as f32) as usize);
        nums.push(function(i as f32 + 0.3) as usize);
        nums.push(function(i as f32 + 0.7) as usize);
    }
    get_plot(nums)
}

pub fn get_letter_plot(plot_data: PlotData) -> String {
    let is_wpm = {
        if plot_data.letter.is_none() {
            if let PlotType::AllWpm = plot_data.plot_type {
                true
            } else {
                false
            }
        } else {
            if let PlotType::LetterWpm = plot_data.plot_type {
                true
            } else {
                false
            }
        }
    };

    let letter = plot_data.letter.unwrap_or(String::from("all"));
    let nums = get_letter_data(&letter, is_wpm);

    get_plot(nums)
}

pub fn get_plot(mut nums: Vec<usize>) -> String {
    let size_res = crossterm::terminal::size();
    let mut size = (80, 40);

    if size_res.is_ok() {
        size = size_res.unwrap();
        let width = (size.0 as f32 / 2.4f32) as u16;
        size.0 = width;
        size.1 -= 5;
    }

    let biggest_num = (*nums.iter().max().unwrap_or(&0)) as u16;

    if biggest_num >= size.1 {
        let scaler = find_scaler(biggest_num, size.1);
        nums = nums
            .iter()
            .cloned()
            .map(|x| (x as f32 * scaler) as usize)
            .collect();
    }

    let length_of_nums = nums.len() as u16;

    let nums = if length_of_nums >= size.0 {
        let scaler = find_scaler(length_of_nums, size.0);
        let max_length_tolerated = (length_of_nums as f32 * scaler - 1f32) as usize;
        let amount_to_remove = length_of_nums - max_length_tolerated as u16;

        let arr = nums.as_mut_slice();

        equal_remove(arr, 0, arr.len(), amount_to_remove, false);

        let mut new_nums = vec![];

        for el in arr {
            if *el == usize::MAX {
                continue;
            }
            new_nums.push(*el);
        }
        new_nums
    } else {
        nums
    };

    let mut plot = Plot::new(size.0.into(), size.1.into());

    if nums.len() < 2 {
        println!("Too few numbers.");
        std::process::exit(1);
    }

    let mut y = *nums.first().unwrap();
    let mut x = 0;

    let x_diff = size.0 as usize / nums.len();

    for num_index in 1..nums.len() - 1 {
        let num = nums[num_index];
        let res = plot.make_diagonal(Coord::new(x, y), Coord::new(x + x_diff, num));
        if res.is_err() {
            println!("{:?}", res.err());
            y = (size.1 / 2) as usize;
            x += 1;
            continue;
        }
        x += x_diff;
        y = num;
    }

    format!("{}", plot)
}
