// fields.rs contains functions which give access to the sudoku field.
// Jonathan KLimt


#[derive(Debug,  Clone, PartialEq)]
pub enum Entry{
    Value (u8),
    // Possibilities ([u8; 9]),
    Possibilities (Vec<u8>),
}

impl Default for Entry{
    fn default() -> Entry {Entry::Possibilities(vec![1,2,3,4,5,6,7,8,9])}
}


pub fn print_playground (pg: & [[Entry ; 9] ; 9], debug: bool){
    println!("+-------+-------+-------+");
    for y in 0..9 {
        for x in 0..9 {
            if x % 3 == 0 {
                print!("| ")
            }
            match pg[x][y]{
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


/// Checks if a Playground is valid, i.e. no Values are Duplicate and no
/// Possibilities Entry exists which is invalid
pub fn is_valid(pg: &[[Entry; 9]; 9]) -> bool {

    for outer in pg {
        for inner in outer {
            print!("inne:{:?} outer:{:?}", inner, outer);
        }
        println!(" ");
        // println!("blah: {:?}", outer);
    }
    return true;
}


pub fn valid_line(line: & [Entry ; 9]) -> bool {
    println!("Line: {:?}", line);
    for i in 1..8 {
        if let Entry::Value(val) = line [i] {
            for j in (i+1)..9{
                match line[j]{
                    Entry::Value(valj) =>
                        if val == valj{
                            return false;
                        },
                     Entry::Possibilities(ref gvec) => {
                         if gvec.iter().find(|&&x| x == val) == Some(&val){
                             return false;
                         }
                     },
                }
            }
        }
    }
    return true;
}


fn remove_from_guesses_column(number: u8, pg: &mut [[Entry ; 9] ; 9], col: usize){
    for i in 0..9 {
        match pg[col][i]{
            Entry::Value(x) => if x == number {
                panic!("Value {} is already in this column", number);
            },
            Entry::Possibilities(ref mut vec) => {
                vec.retain(|x| *x != number);
            },
        }
    }
}

fn remove_from_guesses_line(number: u8, pg: &mut [[Entry ; 9] ; 9], line: usize){
    for i in 0..9 {
        match pg[i][line]{
            Entry::Value(x) => if x == number {
                panic!("Value {} is already in this line", number);
            },
            Entry::Possibilities(ref mut vec) => {
                vec.retain(|x| *x != number);
            },
        }
    }
}

fn remove_from_guesses_square(number: u8, pg: &mut [[Entry ; 9] ; 9],
                              square_x: usize, square_y: usize){
    for x in (square_x * 3)..((square_x * 3) + 3){
        for y in (square_y * 3)..((square_y * 3) + 3){
            match pg[x][y]{
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


pub fn insert_number(number:u8, mut pg: &mut [[Entry ; 9] ; 9],
                     x_coord: usize, y_coord: usize){
    if number == 0 || number > 9 {
        panic!("Error: Trying to insert invalid number {} at position [{}][{}]",
               number, x_coord, y_coord);
    }
    match pg[x_coord][y_coord]{
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
    remove_from_guesses_column(number, &mut pg, x_coord);
    remove_from_guesses_line(number, &mut pg, y_coord);
    let square_x = x_coord/3;
    let square_y = y_coord/3;
    remove_from_guesses_square(number, &mut pg, square_x, square_y);
    pg[x_coord][y_coord] = Entry::Value(number)
}



#[cfg(test)]
mod valid_line_test{
    use super::*;

    #[test]
    fn valid_line_test() {
        let mut testline: [Entry; 9] = Default::default();
        assert_eq!(valid_line(&testline ), true);
        println!("----------");
        testline [2] = Entry::Value(2);
        assert_eq!(valid_line(&testline), false);
        println!("----------");
        testline [3] = Entry::Value(2);
        assert_eq!(valid_line(&testline), false)
    }
}


#[cfg(test)]
mod insert_number_test {

    use super::*;
    static TESTNR : u8 = 8;
    static X : usize = 2;
    static Y : usize = 2;

    #[test]
    fn basic_test() {
        let mut playground: [[Entry; 9]; 9] = Default::default();
        assert!(is_valid(&playground));
        insert_number(TESTNR, &mut playground, X, Y);
        assert!(match playground[X][Y] {
            Entry::Value(i) => if i == TESTNR {true} else {false},
            Entry::Possibilities(..) => false,
        })
    }
    #[test]
    #[should_panic]
    fn insert_twice() {
        let mut playground: [[Entry; 9]; 9] = Default::default();
        assert!(is_valid(&playground));
        insert_number(TESTNR, &mut playground, X, Y);
        insert_number((TESTNR + 1) % 9 + 1, &mut playground, X, Y);
    }
    #[test]
    #[should_panic]
    fn insert_out_of_bound() {
        let mut playground: [[Entry; 9]; 9] = Default::default();
        assert!(is_valid(&playground));
        insert_number(TESTNR, &mut playground, X, Y + 9);
    }
    #[test]
    #[should_panic]
    fn insert_zero() {
        let mut playground: [[Entry; 9]; 9] = Default::default();
        assert!(is_valid(&playground));
        insert_number(TESTNR + 9, &mut playground, X, Y + 9);
    }

    #[test]
    #[should_panic]
    fn insert_invalid_large_number() {
        let mut playground: [[Entry; 9]; 9] = Default::default();
        assert!(is_valid(&playground));
        insert_number(TESTNR + 9, &mut playground, X, Y + 9);
    }

    #[test]
    fn possibilities_constistent(){
        let mut refvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut playground: [[Entry; 9]; 9] = Default::default();
        assert!(is_valid(&playground));
        insert_number(TESTNR, &mut playground, X, Y);
        refvec.retain(|x| *x != TESTNR);
        assert!(playground[X][(Y + 1) % 9 + 1] ==  Entry::Possibilities(refvec.clone())); // Same column
        assert!(playground[(X + 1) % 9 + 1][Y] ==  Entry::Possibilities(refvec.clone())); // Same row
        let x : usize;
        let y : usize;
        if X % 3 != 2 {x = X + 1} else {x = X - 1};
        if Y % 3 != 2 {y = Y + 1} else {y = Y - 1};
        println!("x: {} y: {}", x, y);
        assert!(playground[x][y] ==  Entry::Possibilities(refvec.clone())); // Same square
    }
}
