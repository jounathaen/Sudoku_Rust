// fields.rs contains functions which give access to the sudoku field.
// Jonathan KLimt

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Sudoku{
    field : [[Entry ; 9] ; 9],
    // difficulty : u8,
}

impl Sudoku {
    // pub fn set_difficulty (&mut self, diff : u8){
    //     self.difficulty = diff;
    // }

    /// Checks if a Sudoku is valid, i.e. no Values are Duplicate and no
    /// Possibilities Entry exists which is invalid.
    /// panics if sudoku is not valid
    pub fn check_validity(&self) {
        for y in 0..9 {
            for x in 0..9 {
                match self.field[x][y].clone(){
                    Entry::Value(i) => {
                        if (self.check_col_for_number(i, x) != 1) ||
                            (self.check_line_for_number(i, y) != 1) ||
                            (self.check_square_for_number(i, x/3, y/3) != 1) {
                                panic!("Number {} at [{}][{}], is already present\
                                        in this Sudoku", i, x, y);
                            }
                    },
                    Entry::Possibilities(pvec) => {
                        for i in pvec.iter() {
                            if ( self.check_col_for_number(*i, x) != 0) ||
                                (self.check_line_for_number(*i, y) != 0) ||
                                (self.check_square_for_number(*i, x/3, y/3) != 0) {
                                    panic!("Number {} is a possibility at [{}][{}], \
                                            but this is not possible here", i, x, y);
                                }
                        }
                    },
                }
            }
        }
    }

    fn check_col_for_number(&self, number: u8, col: usize) -> u8 {
        let mut cnt = 0;
        for i in 0..9 {
            match self.field[col][i]{
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
            match self.field[i][line]{
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
                match self.field[x][y]{
                    Entry::Value(i) => if i == number {
                        cnt = cnt + 1;
                    },
                    Entry::Possibilities(..) => {},
                }
            }
        }
        return cnt;
    }

    pub fn print(&self, debug: bool){
        println!("+-------+-------+-------+");
        for y in 0..9 {
            for x in 0..9 {
                if x % 3 == 0 {
                    print!("| ")
                }
                match self.field[x][y]{
                    Entry::Value(i) => print!("{} ", i),
                    Entry::Possibilities(ref pvec) =>
                        if debug == true {print!("{:?} ", pvec);} else { print!("  "); },
                }
            }
            println!("|");
            if y % 3 == 2 {
                println!("+-------+-------+-------+");
            }
        }
    }


    /// Inserts a number at the given Coordinates. Note: Coordinates start at 0 and
    /// end at 8. Number must be between 1 and 9
    pub fn insert_number(&mut self, number:u8,
                         x_coord: usize, y_coord: usize){
        if number == 0 || number > 9 {
            panic!("Error: Trying to insert invalid number {} at position [{}][{}]",
                   number, x_coord, y_coord);
        }
        match self.field[x_coord][y_coord]{
            Entry::Value(x) => panic!("Try to insert {} at [{}][{}], \
                                       but field already contains {}",
                                      number, x_coord, y_coord, x),
            Entry::Possibilities(ref mut gvec) =>
                match gvec.iter().position(|&x| x == number) {
                    None => panic!("Try to insert {} at [{}][{}], \
                                    but this is not possible here",
                                   number, x_coord, y_coord),
                    Some(..) => {gvec.clear();},
                }
        }
        // Remove the Number from the possibilities Vectors
        self.remove_from_guesses_col(number, x_coord);
        self.remove_from_guesses_line(number, y_coord);
        let square_x = x_coord/3;
        let square_y = y_coord/3;
        self.remove_from_guesses_square(number, square_x, square_y);
        self.field[x_coord][y_coord] = Entry::Value(number)
    }

    fn remove_from_guesses_col(&mut self, number: u8, col: usize){
        for i in 0..9 {
            match self.field[col][i]{
                Entry::Value(x) => if x == number {
                    panic!("Value {} is already in this column", number);
                },
                Entry::Possibilities(ref mut vec) => {
                    vec.retain(|x| *x != number);
                },
            }
        }
    }

    fn remove_from_guesses_line(&mut self, number: u8, line: usize){
        for i in 0..9 {
            match self.field[i][line]{
                Entry::Value(x) => if x == number {
                    panic!("Value {} is already in this line", number);
                },
                Entry::Possibilities(ref mut vec) => {
                    vec.retain(|x| *x != number);
                },
            }
        }
    }

    fn remove_from_guesses_square(&mut self, number: u8,
                                  square_x: usize, square_y: usize){
        for x in (square_x * 3)..((square_x * 3) + 3){
            for y in (square_y * 3)..((square_y * 3) + 3){
                match self.field[x][y]{
                    Entry::Value(x) => if x == number {
                        panic!("Value {} is already in this square", number);
                    },
                    Entry::Possibilities(ref mut vec) => {
                        vec.retain(|x| *x != number);
                    },
                }
            }
        }
    }

    pub fn solve_obvious(&mut self) -> u8{
        let mut changecount : u8 = 0;
        for y in 0..9{
            for x in 0..9{
                if let Entry::Possibilities(ref pvec) = self.field[x][y].clone(){
                    if pvec.len() == 1 {
                        self.insert_number(pvec[0], x, y);
                        changecount = changecount + 1;
                    }
                }
            }
        }
        return changecount;
    }


    /// Create a Sudoku from a String. String is read left to right and 0 is
    /// threatened as empty field. Performs a validity check
    pub fn read_from_string(&mut self, input : &String){
        let trimed_input = input.trim();
        assert!(trimed_input.len() == 81);
        for (i, c) in  trimed_input.chars().enumerate(){
            if let Some(dig) = c.to_digit(10){
                if dig > 0 && dig <= 9 {
                    let x = i % 9;
                    let y = i / 9;
                    self.insert_number(dig as u8, x, y);
                }
            }
            else {
                panic!("String {} contains non digit value at {}", input, i);
            }
        }
        self.check_validity();
    }

    pub fn is_solved (&self) -> bool {
        for y in 0..9 {
            for x in 0..9 {
                if let Entry::Possibilities(..) = self.field[y][x] {
                    return false;
                }
            }
        }
        return true;
    }


    pub fn easy_solve(&mut self) -> bool {
        let mut count = 0;
        while {self.solve_obvious() > 0 } {
            count = count + 1;
            println!("Round {}", count);
            self.print(false);
        }

        if self.is_solved() {
            println!("Hooray!!! Solved Sudoku!");
            self.print(false);
            return true;
        }
        else {
            println!("====== Recursion: Going Deeper... ======");
            let mut posvec : Vec<(usize, usize)> = Vec::new();
            // Building posvec (collecting all unsolved positions)
            for y in 0..9 {
                for x in 0..9 {
                    if let Entry::Possibilities(..) = self.field[x][y] {
                        posvec.push((x, y));
                    }
                }
            }
            for i in posvec {
                if let Entry::Possibilities(mut pvec) = self.field[i.0][i.1].clone(){
                    while let Some(probe_number) = pvec.pop(){
                        let mut newsud : Sudoku = self.clone();
                        println!("trying {:?} at {:?}", probe_number, i);
                        newsud.insert_number(probe_number, i.0, i.1);
                        if newsud.easy_solve() {
                            return true;
                        }
                        newsud.print(true);
                    }
                }
            }
        }
        panic!("Can't solve Sudoku");
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





#[cfg(test)]
mod test {

    use super::*;
    static TESTNR : u8 = 8;
    static X : usize = 2;
    static Y : usize = 2;


    #[test]
    fn valid_field(){
        let sud: Sudoku = Default::default();
        sud.print(true);
        sud.check_validity();
    }

    #[test]
    #[should_panic]
    fn invalid_field() {
        let mut sud: Sudoku = Default::default();
        sud.field[X][Y] = Entry::Value(TESTNR);
        sud.check_validity();
    }


    #[test]
    fn insert_number() {
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR, X, Y);
        assert!(match sud.field[X][Y] {
            Entry::Value(i) => if i == TESTNR {true} else {false},
            Entry::Possibilities(..) => false,
        })
    }

    #[test]
    #[should_panic]
    fn insert_twice() {
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR, X, Y);
        sud.insert_number((TESTNR + 1) % 9 + 1, X, Y);
    }

    #[test]
    #[should_panic]
    fn insert_out_of_bound() {
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR, X, Y + 9);
    }

    #[test]
    #[should_panic]
    fn insert_zero() {
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR + 9, X, Y + 9);
    }

    #[test]
    #[should_panic]
    fn insert_invalid_large_number() {
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR + 9, X, Y + 9);
    }

    // TODO: Split this test, as the functionality was also split
    #[test]
    fn possibilities_constistent(){
        let mut refvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR, X, Y);
        refvec.retain(|x| *x != TESTNR);
        assert!(sud.field[X][(Y + 1) % 9 + 1]
                == Entry::Possibilities(refvec.clone())); // Same column
        assert!(sud.field[(X + 1) % 9 + 1][Y]
                == Entry::Possibilities(refvec.clone())); // Same row
        let x : usize;
        let y : usize;
        if X % 3 != 2 {x = X + 1} else {x = X - 1};
        if Y % 3 != 2 {y = Y + 1} else {y = Y - 1};
        // println!("x: {} y: {}", x, y);
        assert!(sud.field[x][y] ==  Entry::Possibilities(refvec.clone())); // Same square
    }

    #[test]
    fn print_sudoku(){
        let mut sud: Sudoku = Default::default();
        sud.insert_number(TESTNR, X, Y);
        sud.print(true);
        sud.print(false);
    }

    #[test]
    fn remove_guesses_col(){
        let mut refvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        refvec.retain(|x| *x != TESTNR);
        let mut sud: Sudoku = Default::default();
        sud.remove_from_guesses_col(TESTNR, X);
        assert!(sud.field[X][(Y + 1) % 9] == Entry::Possibilities(refvec));
    }

    #[test]
    fn remove_guesses_line(){
        let mut refvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        refvec.retain(|x| *x != TESTNR);
        let mut sud: Sudoku = Default::default();
        sud.remove_from_guesses_line(TESTNR, X);
        assert!(sud.field[(X + 1) % 9][Y] == Entry::Possibilities(refvec));
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

        sud.remove_from_guesses_square(TESTNR, X/3, Y/3);
        assert!(sud.field[x][y] == Entry::Possibilities(refvec));
    }

    #[test]
    fn solve_obvious(){
        let mut sud: Sudoku = Default::default();
        sud.insert_number(1,0,0);
        sud.insert_number(2,0,1);
        sud.insert_number(3,0,2);
        sud.insert_number(4,1,0);
        sud.insert_number(5,1,1);
        sud.insert_number(6,1,2);
        sud.insert_number(7,2,0);
        sud.insert_number(8,2,1);
        assert!(sud.solve_obvious() == 1);
    }

    #[test]
    fn read_from_string(){
        let mut sud : Sudoku = Default::default();
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
        sud.read_from_string(&stringfield);
        assert!(sud.field[1][0] == Entry::Value(5));
        assert!(sud.field[4][4] == Entry::Value(2));
        assert!(sud.field[5][8] == Entry::Value(7));
    }

    #[test]
    #[should_panic]
    fn read_from_invalid_string(){
        let mut sud : Sudoku = Default::default();
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
        sud.read_from_string(&stringfield);
    }

    #[test]
    #[should_panic]
    fn read_from_invalid_length_string(){
        let mut sud : Sudoku = Default::default();
        let stringfield = String::from(
            "050083007\
             000100400\
             103607204");
        sud.read_from_string(&stringfield);
    }

    #[test]
    fn is_solved(){
        let mut sud : Sudoku = Default::default();
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
        sud.read_from_string(&stringfield);
        assert!(sud.is_solved() == false);
        sud = Default::default();
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
        sud.read_from_string(&stringfield);
        assert!(sud.is_solved() == true);
    }

}
