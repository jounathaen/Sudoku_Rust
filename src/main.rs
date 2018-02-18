extern crate csv;

use std::fs::File;
use std::error::Error;

mod field;
use field::*;



// mod sudoku_easy_1;

fn solve_sudokus(file_path: &String) -> Result<(), Box<Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
            let mut sud: Sudoku = Default::default();
            if let Some(substring) = result.unwrap().get(0){
                println!("{:?}", substring);
                sud.read_from_string(&String::from(substring));
                sud.easy_solve();
            }
    }
    Ok(())
}

fn main() {
    // Init
    let mut sud: Sudoku = Default::default();
    sud.print(false);
    sudoku_easy_1::fill_sudoku(&mut sud);
    sud.check_validity();
    sud.print(false);
    // sud.print(true);
    let mut count = 0;
    while {sud.solve_obvious() > 0 } {
        count = count + 1;
        println!("Round {}", count);
        sud.print(false);
    }

    sud.check_validity();
}
