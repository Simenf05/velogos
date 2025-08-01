use std::{fmt::{self, write}, process::Output};

use crate::plot;


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

        line.replace_range(x..x+1, "*");

        Ok(())
    }


    fn make_diagonal(&mut self, start_coord: Coord, end_coord: Coord) -> Result<(), String> {

        let mut dy: isize = end_coord.y as isize - start_coord.y as isize;
        let mut reflect_y = false;
        if dy < 0 {
            reflect_y = true;
            dy = -dy;
        }

        let mut place_with_reflect_y = |coord: Coord| -> Result<(), String> {
            if reflect_y 
            { self.place_star(Coord::new(coord.x, start_coord.y - (coord.y - start_coord.y)))?; } 
            else 
            { self.place_star(Coord::new(coord.x, coord.y))?; }
            Ok(())
        };

        let dx: isize = end_coord.x as isize - start_coord.x as isize;

        if dy <= dx {
            let mut d: isize = dy - (dx / 2);
            let mut x: usize = start_coord.x;
            let mut y: usize = start_coord.y;

            place_with_reflect_y(Coord::new(x, y))?;
            while x < end_coord.x {
                x += 1;
                if d < 0 {
                    d += dy;
                }
                else {
                    d = d + dy - dx;
                    y += 1;
                }

                place_with_reflect_y(Coord::new(x, y))?;
            }
        }
        else if dx <= dy {

            let mut d: isize = dx - (dy / 2);
            let mut x: usize = start_coord.x;
            let mut y: usize = start_coord.y;


            place_with_reflect_y(Coord::new(x, y))?;
            while y < end_coord.y {
                y += 1;

                if d < 0 {
                    d += dx;
                }
                else {
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

impl fmt::Display for Plot  {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut width = self.width * 2;
        width += self.width / 3;
        if self.width % 3 != 0 {
            width += 1;
        }

        let first_and_last_line = format!("+{:-<width$}+", "");
        let mut grid = self.grid.clone();
        grid = grid.iter_mut().map(|line| insert_spaces(line)).map(|line| format!("|{}|", line)).collect();
        let output = grid.join("\n");
        write!(f, "{first_and_last_line}\n{}\n{first_and_last_line}\n", output)
    }
}


// 9 x 21

pub fn get_plot() -> String {

    // let plot = Plot::new(150, 40);
    let dim = 40;
    let mut plot = Plot::new(dim*2, dim);

    let res = plot.make_diagonal(Coord::new(10, 39), Coord::new(40, 19));


    println!("{}", plot);


    String::new()
}

