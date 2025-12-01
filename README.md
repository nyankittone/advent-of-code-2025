# What is this?
This is a single binary that solves problems from Advent of Code 2025. It is a Rust binary that,
depending on the input args, will either load a Rust function or a C++ function for solving a
problem, while feeding that function with data laoded from a file specified. Passing `-i` to the
program will make it solve the second part of the day.

Each problem will be solved without any dependencies from whatever language is solving the problem,
other than the standard library. This might be problematic in the case of Rust, since it has a
smaller standard library.

Synopsis for running the compiled binary is: `program [-i] <number for the day> [puzzle input file]`

If no puzzle input file is provided, the program will load from stdin instead.

Puzzle inputs are not provided in the repo.
