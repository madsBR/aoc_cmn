use std::{process::Output, fmt::Display};
use crate::file_read_utils::read_input_of_puzzle;


pub trait Solution{
    const PUZZLE_DATE : u64;
    const PUZZLE_NR : u64;
    type Output : Display;
    fn solve() -> Self::Output;

    fn solve_and_print(){
        let sol = Self::solve();
        println!("Solution to puzzle {:02} nr {}: {}",Self::PUZZLE_DATE,Self::PUZZLE_NR,sol)
    }
}