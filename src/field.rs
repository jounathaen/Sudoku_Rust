// fields.rs contains functions which give access to the sudoku field.
// Jonathan Klimt
extern crate csv;
extern crate time;
extern crate termion;

use std::fs::File;
use std::error::Error;
use std::fmt;
use self::termion::{clear, cursor, color, style};
use std::time::Duration;
use std::thread;
use std::convert::From;


static ANIM_SPEED: u64 = 1500;


#[derive(Default, Debug, Clone, PartialEq)]
pub struct Sudoku{
    grid : [[Field ; 9] ; 9], // Rework this, no touple
    pub print_lvl : Lvl,
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Field {
    entry : Entry,
    status : Status,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lvl{
    None,
    Solution,
    Verbose,
    Interactive
}
impl Default for Lvl{
    fn default() -> Lvl {Lvl::None}
}

#[derive(Debug, Clone, PartialEq)]
enum Status{
    Given,
    Inserted
}
impl Default for Status{
    fn default() -> Status {Status::Inserted}
}


impl Sudoku {

    /// Checks if a Sudoku is valid, i.e. no Values are Duplicate and no
    /// Possibilities Entry exists which is invalid.
    /// panics if sudoku is not valid
    pub fn check_validity(&self) -> Result<(), SolvingError> {
        for y in 0..9 {
            for x in 0..9 {
                match self.grid[x][y].entry.clone(){
                    Entry::Value(i) => {
                        if (self.check_col_for_number(i, x) != 1) ||
                            (self.check_line_for_number(i, y) != 1) ||
                            (self.check_square_for_number(i, x/3, y/3) != 1) {
                                return Err(SolvingError::new(
                                    &format!("Number {} at [{}][{}], is already present\
                                             in this Sudoku", i, x, y)));
                            }
                    },
                    Entry::Possibilities(pvec) => {
                        if pvec.len() == 0 {
                            return Err(SolvingError::new(
                                &format!("No possible value at pos [{}][{}]", x, y)));
                        }
                        for i in pvec.iter() {
                            if ( self.check_col_for_number(*i, x) != 0) ||
                                (self.check_line_for_number(*i, y) != 0) ||
                                (self.check_square_for_number(*i, x/3, y/3) != 0) {
                                    return Err(SolvingError::new(
                                        &format!("Number {} is a possibility at [{}][{}], \
                                                 but this is not possible here", i, x, y)));
                                }
                        }
                    },
                }
            }
        }
        return Ok(());
    }

    fn check_col_for_number(&self, number: u8, col: usize) -> u8 {
        let mut cnt = 0;
        for i in 0..9 {
            match self.grid[col][i].entry{
                Entry::Value(x) => if x == number {
                    cnt = cnt + 1;
                },
                Entry::Possibilities(..) => {},
            }
        }
        return cnt;
    }

    fn check_line_for_number(&self, number: u8, line: usize) -> u8 {
        let mut cnt = 0;
        for i in 0..9 {
            match self.grid[i][line].entry{
                Entry::Value(x) => if x == number {
                    cnt = cnt + 1;
                },
                Entry::Possibilities(..) => {},
            }
        }
        return cnt;
    }

    fn check_square_for_number(&self, number: u8,
                               square_x: usize, square_y: usize) -> u8 {
        let mut cnt = 0;
        for x in (square_x * 3)..((square_x * 3) + 3){
            for y in (square_y * 3)..((square_y * 3) + 3){
                match self.grid[x][y].entry{
                    Entry::Value(i) => if i == number {
                        cnt = cnt + 1;
                    },
                    Entry::Possibilities(..) => {},
                }
            }
        }
        return cnt;
    }


    /// Inserts a number at the given Coordinates. Note: Coordinates start at 0 and
    /// end at 8. Number must be between 1 and 9
    pub fn insert_number(&mut self, number:u8,
                         x_coord: usize, y_coord: usize) -> Result<(), Box<Error>>{
        if number == 0 || number > 9 {
            return Err(From::from(InsertError::new(x_coord, y_coord)));
        }
        match self.grid[x_coord][y_coord].entry{
            Entry::Value(..) => return Err(From::from(InsertError::new(x_coord, y_coord))),
            Entry::Possibilities(ref mut gvec) =>
                match gvec.iter().position(|&x| x == number) {
                    None => return Err(From::from(InsertError::new(x_coord, y_coord))),
                    Some(..) => {gvec.clear();},
                }
        }
        // Remove the Number from the possibilities Vectors
        self.remove_from_guesses_col(number, x_coord)?;
        self.remove_from_guesses_line(number, y_coord)?;
        let square_x = x_coord/3;
        let square_y = y_coord/3;
        self.remove_from_guesses_square(number, square_x, square_y)?;
        self.grid[x_coord][y_coord].entry = Entry::Value(number);
        Ok(())
    }

    fn remove_from_guesses_col(&mut self, number: u8, col: usize)
                               -> Result<(), Box<Error>>{
        let mut empty_field_count = 0;
        for i in 0..9 {
            match self.grid[col][i].entry{
                Entry::Value(x) => if x == number {
                    return Err(From::from(InsertError::new(i, col)));
                },
                Entry::Possibilities(ref mut vec) => {
                    vec.retain(|x| *x != number);
                    if vec.len() == 0 {
                        empty_field_count += 1;
                    }


                },
            }
        }
        if empty_field_count > 1 {
            return Err(From::from(SolvingError::new("no possible values left")))
        }
        else {
            Ok(())
        }

    }

    fn remove_from_guesses_line(&mut self, number: u8, line: usize)
                                -> Result<(), Box<Error>>{
        let mut empty_field_count = 0;
        for i in 0..9 {
            match self.grid[i][line].entry{
                Entry::Value(x) => if x == number {
                    return Err(From::from(InsertError::new(line, i)));
                },
                Entry::Possibilities(ref mut vec) => {
                    vec.retain(|x| *x != number);
                    if vec.len() == 0 {
                        empty_field_count += 1;
                    }
                },
            }
        }
        if empty_field_count > 1 {
            return Err(From::from(SolvingError::new("no possible values left")))
        }
        else {
            Ok(())
        }
    }

    //TODO should fail if the posvec is empty, but should not fail for insert value at that point
    fn remove_from_guesses_square(&mut self, number: u8,
                                  square_x: usize, square_y: usize)
                                  -> Result<(), Box<Error>>{
        let mut empty_field_count = 0;
        for x in (square_x * 3)..((square_x * 3) + 3){
            for y in (square_y * 3)..((square_y * 3) + 3){
                match self.grid[x][y].entry{
                    Entry::Value(i) => if i == number {
                        return Err(From::from(InsertError::new(x, y)));
                    },
                    Entry::Possibilities(ref mut vec) => {
                        vec.retain(|x| *x != number);
                        if vec.len() == 0 {
                            empty_field_count += 1;
                        }
                    },
                }
            }
        }
        if empty_field_count > 1 {
            return Err(From::from(SolvingError::new("no possible values left")))
        }
        else {
            Ok(())
        }

    }

    pub fn solve_obvious(&mut self) -> Result<u8, SolvingError>{
        let mut changecount : u8 = 0;
        for y in 0..9{
            for x in 0..9{
                if let Entry::Possibilities(ref pvec) = self.grid[x][y].entry.clone(){
                    if pvec.len() == 1 {
                        match self.insert_number(pvec[0], x, y){
                            Err(..) => return Err(SolvingError::new(&format!(
                                "Could not easy-solve Sudoku. Can not insert \
                                 Number at x:{} y:{}", x, y))),
                            Ok(()) => {},
                        }
                        changecount = changecount + 1;
                    }
                }
            }
        }
        return Ok(changecount);
    }



    pub fn is_solved (&self) -> bool {
        for y in 0..9 {
            for x in 0..9 {
                if let Entry::Possibilities(..) = self.grid[y][x].entry {
                    return false;
                }
            }
        }
        return true;
    }


    // pub fn easy_solve(&mut self, lvl: u64) -> Result<(), SolvingError> {
    pub fn easy_solve(&mut self) -> Result<(), SolvingError> {
        if self.print_lvl == Lvl::Interactive{
            println!("{}{}Now Solve!{}",clear::All, cursor::Goto(1,1), self);
            thread::sleep(Duration::from_millis(ANIM_SPEED));
        }
        let mut count = 0;
        let mut changes = 1;
        while {changes > 0 } {
            if self.print_lvl == Lvl::Verbose {
                println!("Round {}{}", count, self);
            }
            if self.print_lvl == Lvl::Interactive{
                print!("{}{}Round {}{}", clear::All, cursor::Goto(1,1), count, self);
                thread::sleep(Duration::from_millis(ANIM_SPEED));
            }
            count = count + 1;
            match self.solve_obvious(){
                Ok (i) => changes = i,
                Err(err) => return Err(err),
            }
            // TODO solve not so obvious ones, where i is the only appearance in a line or Block
        }

        if self.is_solved() {
            if self.print_lvl != Lvl::None{
                println!("Hooray!!! Solved Sudoku! {}", self);
            }
            return Ok(());
        }
        else {
            if self.print_lvl == Lvl::Verbose {
                println!("====== Recursion: Going Deeper... ======");
            }

            let mut first_choice = (0,0);
            'yloop: for y in 0..9 {
                for x in 0..9 {
                    if let Entry::Possibilities(..) = self.grid[x][y].entry {
                        first_choice = (x, y);
                        break 'yloop;
                    }
                }
            }
            if let Entry::Possibilities(mut pvec) = self.grid[first_choice.0][first_choice.1].entry.clone(){
                while let Some(probe_number) = pvec.pop(){
                    let mut newsud : Sudoku = self.clone();
                    match newsud.insert_number(probe_number, first_choice.0, first_choice.1){
                        Err(..) => {println!(">> Didn't work. DANG!\n\n\n"); break},
                        Ok(..) => {},
                    }

                    match self.print_lvl {
                        Lvl::Verbose => {
                            println!("trying {:?} at {:?}", probe_number, first_choice);},
                        Lvl::Interactive => {
                            println!("{}{}trying {:?} at {:?}{}", clear::All,
                                     cursor::Goto(1,1), probe_number, first_choice, self);
                            thread::sleep(Duration::from_millis(ANIM_SPEED));
                        }
                        Lvl::None => {},
                        Lvl::Solution => {},
                    }
                    if let Ok(..) = newsud.easy_solve() {
                        self.grid = newsud.grid;
                        return Ok(());
                    }
                    if self.print_lvl == Lvl::Verbose || self.print_lvl == Lvl::Interactive {
                        println!(">>>>>>> Nope, let's try another one <<<<<<<<<");
                    }
                }
            }
        }
        Err(SolvingError::new("Can't solve Sudoku"))
    }

}

impl fmt::Display for Sudoku{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.print_lvl == Lvl::Verbose || self.print_lvl == Lvl::Interactive{
            write!(f, "\n ┏━━━━━┯━━━━━┯━━━━━┳━━━━━┯━━━━━┯━━━━━┳━━━━━┯━━━━━┯━━━━━┓\n")?;
            for y in 0..9 {
                for y1 in 0..3 {
                    for x in 0..9 {
                        if x % 3 == 0 {
                            write!(f, " ┃ ")?;
                        } else {
                            write!(f, " │ ")?;
                        }

                        match self.grid[x][y].entry{
                            Entry::Value(i) =>
                                if y1 == 1 {
                                    if self.grid[x][y].status == Status::Given{
                                        write!(f, " {}{}{}{}{} ", style::Bold,
                                               color::Fg(color::LightCyan), i,
                                               color::Fg(color::Reset), style::Reset)?;
                                    }else{
                                        write!(f, " {}{}{}{}{} ",style::Bold,
                                               color::Fg(color::Red), i,
                                               color::Fg(color::Reset), style::Reset)?;
                                    }
                                } else {
                                    write!(f, "   ")?;
                                },
                            Entry::Possibilities(ref pvec) => {
                                write!(f, "{}", color::Fg(color::LightBlack))?;
                                for x1 in 1..4 {
                                    if pvec.contains(&((x1 + y1 * 3) as u8)) {
                                        write!(f, "{}", x1 + y1 * 3)?;
                                    }
                                    else{
                                        write!(f, " ")?;
                                    }
                                }
                                write!(f, "{}", color::Fg(color::Reset))?
                            },
                        }
                    }
                    write!(f, " ┃\n")?;
                }
                if y != 8 {
                    if y % 3 == 2 {
                        write!(f, " ┣━━━━━┿━━━━━┿━━━━━╋━━━━━┿━━━━━┿━━━━━╋━━━━━┿━━━━━┿━━━━━┫\n")?;
                    } else {
                        write!(f, " ┠─────┼─────┼─────╂─────┼─────┼─────╂─────┼─────┼─────┨\n")?;
                    }
                }
            }
            write!(f, " ┗━━━━━┷━━━━━┷━━━━━┻━━━━━┷━━━━━┷━━━━━┻━━━━━┷━━━━━┷━━━━━┛\n")
        }
        else{
            write!(f, "\n ┏━━━┯━━━┯━━━┳━━━┯━━━┯━━━┳━━━┯━━━┯━━━┓\n")?;
            for y in 0..9 {
                for x in 0..9 {
                    if x % 3 == 0 {
                        write!(f, " ┃ ")?;
                    } else {
                        write!(f, " │ ")?;
                    }
                    match self.grid[x][y].entry{
                        Entry::Value(i) => write!(f, "{}", i)?,
                        Entry::Possibilities(..) =>
                            write!(f, " ")?,
                    }
                }
                write!(f, " ┃\n")?;
                if y != 8 {
                    if y % 3 == 2 {
                        write!(f, " ┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫\n")?;
                    } else {
                        write!(f, " ┠───┼───┼───╂───┼───┼───╂───┼───┼───┨\n")?;
                    }
                }
            }
            write!(f, " ┗━━━┷━━━┷━━━┻━━━┷━━━┷━━━┻━━━┷━━━┷━━━┛\n")
        }
    }
}

/// Create a Sudoku from a String. String is read left to right and 0 is
/// threatened as empty field. Performs a validity check
//TODO use TryFrom, once it is stable
impl From<String> for Sudoku {
    fn from (input : String) -> Sudoku {
    // pub fn read_from_string(&mut self, input : &String) -> Result<(), Box<Error>> {
        let mut sud: Sudoku = Default::default();
        let trimed_input = input.trim();
        assert!(trimed_input.len() == 81);
        for (i, c) in  trimed_input.chars().enumerate(){
            if let Some(dig) = c.to_digit(10){
                if dig > 0 && dig <= 9 {
                    let x = i % 9;
                    let y = i / 9;
                    sud.insert_number(dig as u8, x, y).unwrap();
                    sud.grid[x][y].status = Status::Given;
                }
            }
            else {
                panic!("Sudoku String contains invalid Characters");
                // TODO return ioerror
                // this is for a possible try_from implementation
                //     return Err(From::from(SolvingError::new(
                //         &format!("String {} contains non digit value at {}", input, i))));
            }
        }
        match sud.check_validity(){
            // Err(e) => Err(From::from(e)),
            // Ok(()) => Ok(()),
            Err(..) => panic!("Invalid Sudoku"),
            Ok(()) => {},
        }
        return sud;
    }
}

#[derive(Debug,  Clone, PartialEq)]
enum Entry{
    Value (u8),
    // Possibilities ([u8; 9]),
    Possibilities (Vec<u8>),
}

impl Default for Entry{
    fn default() -> Entry {Entry::Possibilities(vec![1,2,3,4,5,6,7,8,9])}
}


#[derive(Debug)]
pub struct SolvingError {
    details: String
}

impl SolvingError {
    fn new(msg: &str) -> SolvingError {
        SolvingError{details: msg.to_string()}
    }
}

impl fmt::Display for SolvingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for SolvingError {
    fn description(&self) -> &str {
        &self.details
    }
}


#[derive(Debug)]
pub struct InsertError {
    x: usize,
    y: usize
}

impl InsertError {
    fn new(ex: usize, ey: usize) -> InsertError {
        InsertError{x: ex, y: ey}
    }
}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot insert at position x:{} y:{}. \
                   Either there already exists some value here, or the value to \
                   insert is not possible at that location.", self.x, self.y)
    }
}

impl Error for InsertError {
    fn description(&self) -> &str {
        "Inserting a value to the Sudoku has failed"
    }
}

//TODO Write test
pub fn solve_sudokus_from_csv(file_path: &String) -> Result<(), Box<Error>> {
    let mut counter : u64 = 1;
    let beginn_time= time::precise_time_ns();
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let mut sud: Sudoku;
        if let Some(substring) = result.unwrap().get(0){
            let start = time::precise_time_ns();
            // sud.read_from_string(&String::from(substring))?;
            sud  = Sudoku::from(String::from(substring));
            if sud.print_lvl != Lvl::None {
                println!("{:?}", substring);
                println!("Sudoku: {}", sud);
            }

            match sud.easy_solve(){
                Err(..) => panic!("Unsolvable Sudoku"),
                Ok(()) => {},
            }
            if sud.print_lvl != Lvl::None {
                println!("solved Sudoku: {}", sud);
            }
            let diff : f64 = (time::precise_time_ns() - start) as f64 / 1000000.0;
            println!("took {} ms to solve Sudoku {}", diff, counter);
            counter = counter + 1;
        }
    }
    let diff : f64 = (time::precise_time_ns() - beginn_time) as f64 / 1000000.0;
    println!("took {} ms to solve all Sudokus", diff);
    Ok(())
}


#[cfg(test)]
mod test {

    use super::*;
    static TESTNR : u8 = 8;
    static X : usize = 2;
    static Y : usize = 2;


    #[test]
    fn valid_grid(){
        let sud: Sudoku = Default::default();
        sud.check_validity().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_grid() {
        let mut sud: Sudoku = Default::default();
        sud.grid[X][Y].entry = Entry::Value(TESTNR);
        sud.check_validity().unwrap();
    }

    #[test]
    #[should_panic]
    fn field_double_number() {
        let mut _sud: Sudoku = Default::default();
        _sud = Sudoku::from(String::from(
            "503070190\
             000006750\
             047190600\
             400038000\
             950200300\
             000010072\
             000804001\
             300001860\
             086720005"));
        _sud.grid[1][0].entry = Entry::Value(5);
        _sud.check_validity().unwrap();
    }


    #[test]
    fn insert_number() {
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR, X, Y).unwrap();
        assert!(match sud.grid[X][Y].entry {
            Entry::Value(i) => if i == TESTNR {true} else {false},
            Entry::Possibilities(..) => false,
        })
    }

    #[test]
    #[should_panic]
    fn insert_twice() {
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR, X, Y).unwrap();
        sud.insert_number((TESTNR + 1) % 9 + 1, X, Y).unwrap();
    }

    #[test]
    #[should_panic]
    fn insert_out_of_bound() {
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR, X, Y + 9).unwrap();
    }

    #[test]
    #[should_panic]
    fn insert_zero() {
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR + 9, X, Y + 9).unwrap();
    }

    #[test]
    #[should_panic]
    fn insert_invalid_large_number() {
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR + 9, X, Y + 9).unwrap();
    }

    // TODO: Split this test, as the functionality was also split
    #[test]
    fn possibilities_constistent(){
        let mut refvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR, X, Y).unwrap();
        refvec.retain(|x| *x != TESTNR);
        assert!(sud.grid[X][(Y + 1) % 9 + 1].entry
                == Entry::Possibilities(refvec.clone())); // Same column
        assert!(sud.grid[(X + 1) % 9 + 1][Y].entry
                == Entry::Possibilities(refvec.clone())); // Same row
        let x : usize;
        let y : usize;
        if X % 3 != 2 {x = X + 1} else {x = X - 1};
        if Y % 3 != 2 {y = Y + 1} else {y = Y - 1};
        // println!("x: {} y: {}", x, y);
        assert!(sud.grid[x][y].entry ==  Entry::Possibilities(refvec.clone())); // Same square
    }

    #[test]
    fn print_sudoku(){
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR, X, Y).unwrap();
        println!("{}", sud);
    }

    #[test]
    fn remove_guesses_col(){
        let mut refvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        refvec.retain(|x| *x != TESTNR);
        let mut sud: Sudoku = Default::default();
        sud.remove_from_guesses_col(TESTNR, X).unwrap();
        assert!(sud.grid[X][(Y + 1) % 9].entry == Entry::Possibilities(refvec));
    }

    #[test]
    fn remove_guesses_line(){
        let mut refvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        refvec.retain(|x| *x != TESTNR);
        let mut sud: Sudoku = Default::default();
        sud.remove_from_guesses_line(TESTNR, X).unwrap();
        assert!(sud.grid[(X + 1) % 9][Y].entry == Entry::Possibilities(refvec));
    }

    #[test]
    fn remove_guesses_square(){
        let mut sud: Sudoku = Default::default();
        let mut refvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        refvec.retain(|x| *x != TESTNR);
        let x : usize;
        let y : usize;
        if X % 3 != 2 {x = X + 1} else {x = X - 1};
        if Y % 3 != 2 {y = Y + 1} else {y = Y - 1};

        sud.remove_from_guesses_square(TESTNR, X/3, Y/3).unwrap();
        assert!(sud.grid[x][y].entry == Entry::Possibilities(refvec));
    }

    #[test]
    fn solve_obvious(){
        let mut sud: Sudoku = Default::default();
        sud.insert_number(1,0,0).unwrap();
        sud.insert_number(2,0,1).unwrap();
        sud.insert_number(3,0,2).unwrap();
        sud.insert_number(4,1,0).unwrap();
        sud.insert_number(5,1,1).unwrap();
        sud.insert_number(6,1,2).unwrap();
        sud.insert_number(7,2,0).unwrap();
        sud.insert_number(8,2,1).unwrap();
        assert!(sud.solve_obvious().unwrap() == 1);
    }

    #[test]
    fn read_from_string(){
        let mut _sud : Sudoku = Default::default();
        let stringfield = String::from(
            "   050083017\
             000100400\
             304005608\
             000030009\
             090824500\
             006000070\
             009000050\
             007290086\
             103607204  ");
        _sud = Sudoku::from(stringfield);
        assert!(_sud.grid[1][0].entry == Entry::Value(5));
        assert!(_sud.grid[4][4].entry == Entry::Value(2));
        assert!(_sud.grid[5][8].entry == Entry::Value(7));
    }

    #[test]
    #[should_panic]
    fn read_from_invalid_string(){
        let mut _sud : Sudoku = Default::default();
        let stringfield = String::from(
            "0500830a7\
             000100400\
             XXXXXX608\
             000030009\
             090824500\
             006000070\
             009000050\
             007290086\
             103607204");
        _sud = Sudoku::from(stringfield);
    }

    #[test]
    #[should_panic]
    fn read_from_invalid_length_string(){
        let mut _sud : Sudoku = Default::default();
        let stringfield = String::from(
            "050083007\
             000100400\
             103607204");
        _sud = Sudoku::from(stringfield);
    }

    #[test]
    fn is_solved(){
        let mut _sud : Sudoku = Default::default();
        let mut stringfield = String::from(
            "050083017\
             000100400\
             304005608\
             000030009\
             090824500\
             006000070\
             009000050\
             007290086\
             103607204");
        _sud = Sudoku::from(stringfield);
        assert!(_sud.is_solved() == false);
        _sud = Default::default();
        stringfield = String::from(
            "652483917\
             978162435\
             314975628\
             825736149\
             791824563\
             436519872\
             269348751\
             547291386\
             183657294");
        _sud = Sudoku::from(stringfield);
        assert!(_sud.is_solved() == true);
    }

    #[test]
    fn easy_solve(){
        let mut _sud: Sudoku = Default::default();
        _sud = Sudoku::from(String::from(
            "068700900\
             004000071\
             030809050\
             300080100\
             040005007\
             007304092\
             602001005\
             000020600\
             059030028"));
        _sud.easy_solve().unwrap();
        assert!(_sud.grid[0][0].entry == Entry::Value(5));
        assert!(_sud.grid[0][4].entry == Entry::Value(2));
        assert!(_sud.grid[4][5].entry == Entry::Value(6));
        assert!(_sud.grid[6][8].entry == Entry::Value(7));
        // 568 | 712 | 943
        // 924 | 653 | 871
        // 731 | 849 | 256
        // ---------------
        // 395 | 287 | 164
        // 246 | 195 | 387
        // 817 | 364 | 592
        // ---------------
        // 682 | 971 | 435
        // 473 | 528 | 619
        // 159 | 436 | 728

    }

}
