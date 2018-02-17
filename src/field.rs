// fields.rs contains functions which give access to the sudoku field.
// Jonathan KLimt

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Sudoku{
    field : [[Entry ; 9] ; 9],
    difficulty : u8,
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


pub fn print_sudoku (sud: & Sudoku, debug: bool){
    println!("+-------+-------+-------+");
    for y in 0..9 {
        for x in 0..9 {
            if x % 3 == 0 {
                print!("| ")
            }
            match sud.field[x][y]{
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


/// Checks if a Sudoku is valid, i.e. no Values are Duplicate and no
/// Possibilities Entry exists which is invalid
pub fn is_valid(sud: &Sudoku) -> bool {
    //TODO: IMPLEMENT 
    // for outer in sud.field.iter() {
    //     for inner in outer {
    //         print!("inne:{:?} outer:{:?}", inner, outer);
    //     }
    //     println!(" ");
    //     // println!("blah: {:?}", outer);
    // }
    return true;
}


// pub fn valid_line(sud: & Sudoku, line : usize) -> bool {
//     println!("Line: {:?}", line);
//     for i in 1..9 {
//         if let Entry::Value(val) = sud.field[i][line] {
//             for j in (i+1)..9{
//                 match sud[j]{
//                     Entry::Value(valj) =>
//                         if val == valj{
//                             return false;
//                         },
//                      Entry::Possibilities(ref gvec) => {
//                          if gvec.iter().find(|&&x| x == val) == Some(&val){
//                              return false;
//                          }
//                      },
//                 }
//             }
//         }
//     }
//     return true;
// }


fn remove_from_guesses_column(number: u8, sud: &mut Sudoku, col: usize){
    for i in 0..9 {
        match sud.field[col][i]{
            Entry::Value(x) => if x == number {
                panic!("Value {} is already in this column", number);
            },
            Entry::Possibilities(ref mut vec) => {
                vec.retain(|x| *x != number);
            },
        }
    }
}

fn remove_from_guesses_line(number: u8, sud: &mut Sudoku, line: usize){
    for i in 0..9 {
        match sud.field[i][line]{
            Entry::Value(x) => if x == number {
                panic!("Value {} is already in this line", number);
            },
            Entry::Possibilities(ref mut vec) => {
                vec.retain(|x| *x != number);
            },
        }
    }
}

fn remove_from_guesses_square(number: u8, sud: &mut Sudoku,
                              square_x: usize, square_y: usize){
    for x in (square_x * 3)..((square_x * 3) + 3){
        for y in (square_y * 3)..((square_y * 3) + 3){
            match sud.field[x][y]{
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


pub fn insert_number(number:u8, mut sud: &mut Sudoku,
                     x_coord: usize, y_coord: usize){
    if number == 0 || number > 9 {
        panic!("Error: Trying to insert invalid number {} at position [{}][{}]",
               number, x_coord, y_coord);
    }
    match sud.field[x_coord][y_coord]{
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
    remove_from_guesses_column(number, &mut sud, x_coord);
    remove_from_guesses_line(number, &mut sud, y_coord);
    let square_x = x_coord/3;
    let square_y = y_coord/3;
    remove_from_guesses_square(number, &mut sud, square_x, square_y);
    sud.field[x_coord][y_coord] = Entry::Value(number)
}



// #[cfg(test)]
// mod valid_line_test{
//     use super::*;

//     #[test]
//     fn valid_line_test() {
//         let mut testline: [Entry; 9] = Default::default();
//         assert_eq!(valid_line(&testline ), true);
//         println!("----------");
//         testline [2] = Entry::Value(2);
//         assert_eq!(valid_line(&testline), false);
//         println!("----------");
//         testline [3] = Entry::Value(2);
//         assert_eq!(valid_line(&testline), false)
//     }
// }


#[cfg(test)]
mod insert_number_test {

    use super::*;
    static TESTNR : u8 = 8;
    static X : usize = 2;
    static Y : usize = 2;

    #[test]
    fn basic_test() {
        let mut sudoku: Sudoku = Default::default();
        assert!(is_valid(&sudoku));
        insert_number(TESTNR, &mut sudoku, X, Y);
        assert!(match sudoku.field[X][Y] {
            Entry::Value(i) => if i == TESTNR {true} else {false},
            Entry::Possibilities(..) => false,
        })
    }
    #[test]
    #[should_panic]
    fn insert_twice() {
        let mut sudoku: Sudoku = Default::default();
        assert!(is_valid(&sudoku));
        insert_number(TESTNR, &mut sudoku, X, Y);
        insert_number((TESTNR + 1) % 9 + 1, &mut sudoku, X, Y);
    }
    #[test]
    #[should_panic]
    fn insert_out_of_bound() {
        let mut sudoku: Sudoku = Default::default();
        assert!(is_valid(&sudoku));
        insert_number(TESTNR, &mut sudoku, X, Y + 9);
    }
    #[test]
    #[should_panic]
    fn insert_zero() {
        let mut sudoku: Sudoku = Default::default();
        assert!(is_valid(&sudoku));
        insert_number(TESTNR + 9, &mut sudoku, X, Y + 9);
    }

    #[test]
    #[should_panic]
    fn insert_invalid_large_number() {
        let mut sudoku: Sudoku = Default::default();
        assert!(is_valid(&sudoku));
        insert_number(TESTNR + 9, &mut sudoku, X, Y + 9);
    }

    // TODO: Split this test, as the functionality was also split
    #[test]
    fn possibilities_constistent(){
        let mut refvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut sudoku: Sudoku = Default::default();
        assert!(is_valid(&sudoku));
        insert_number(TESTNR, &mut sudoku, X, Y);
        refvec.retain(|x| *x != TESTNR);
        assert!(sudoku.field[X][(Y + 1) % 9 + 1] ==  Entry::Possibilities(refvec.clone())); // Same column
        assert!(sudoku.field[(X + 1) % 9 + 1][Y] ==  Entry::Possibilities(refvec.clone())); // Same row
        let x : usize;
        let y : usize;
        if X % 3 != 2 {x = X + 1} else {x = X - 1};
        if Y % 3 != 2 {y = Y + 1} else {y = Y - 1};
        println!("x: {} y: {}", x, y);
        assert!(sudoku.field[x][y] ==  Entry::Possibilities(refvec.clone())); // Same square
    }

    #[test]
    fn print_sudoku_test(){
        let mut sudoku: Sudoku = Default::default();
        assert!(is_valid(&sudoku));
        insert_number(TESTNR, &mut sudoku, X, Y);
        print_sudoku(&sudoku, true);
        print_sudoku(&sudoku, false);
    }
}
