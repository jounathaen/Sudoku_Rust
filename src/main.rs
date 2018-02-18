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
    let file_path = "/";
    if let Err(err) = solve_sudokus(&String::from(file_path)) {
        println!("{}", err);
    }
}
