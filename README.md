# Sudoku_Rust
Learning Rust by programming a simple Sudoku solver

Each field in the game grid contains either the right number, or a vector of possible values at that point.
The vectors with the possibilities are updated, everytime a new number is inserted in the grid.

To solve the Sudoku, at first the algorithm solves the obvous fields, where only one number is possible.
After that, the algorithm tries the possibilities recoursively, thous performing a depth first search.


Example Games are taken from https://www.kaggle.com/bryanpark/sudoku/data



## TODO
- Export Sudoku as String
