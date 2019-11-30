# Sudoku

Sudoku is a logic game where the player is presented with a partially filled in grid of 
numbers, and must fill in the remaining spaces such that the numbers 1 through 9 appear in 
each row, column, and 3x3 box exactly once.  In a valid sudoku puzzle there will only be
one way to do this.

Puzzles can be expressed as a sequence of numbers by reading the boxes of the puzzle left
to right and top to bottom such that blank cells are represented by a zero, and filled in 
cells are given their corresponding number.

This program takes a puzzle represented in this format and outputs a list of solutions.