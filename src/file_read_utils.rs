use std::{fs, path::Path, io, fmt::Display};
use itertools::{FoldWhile::{Continue,Done}, Itertools};
use num_traits::{FromBytes, ToPrimitive, FromPrimitive};
/*
loads an int from ascii. returns an error with the non-ascii character if such is found
returns null byte as error if empty string
*/

//convert a single char to integer. Returns the b back in case of error
pub fn atoi<I : FromPrimitive + std::ops::Sub<Output = I> + Ord>(b : u8) -> Result<I,u8>{
    let x = I::from_u8(b);
    //used to unwrap 48 and 58,
    let c48 = I::from_u64(48).unwrap();
    let c58 = I::from_u64(58).unwrap();
    
    match x {
        Some(x) if (x >= c48) && (x < c58) => Ok(x-c48),
        _ => Err(b)
    }
}

pub fn get_int_len(ascii_digits : &[u8]) -> usize {
    let mut res : usize = 0;
    for b in ascii_digits.iter(){
        if let Err(_) = atoi::<u8>(*b){
            return res;
        } else{
            res +=1;
        }
    }
    res    
} 

pub fn parse_int_greedy<I : Clone+ Copy + Display+ FromPrimitive + std::ops::Sub<Output = I> + std::ops::Mul<Output = I> + std::ops::Add<Output = I> + Ord>
(mut ascii_digits : &[u8]) -> Option<I>{
    if ascii_digits.len() == 0 {return None};
    let is_neg : bool = ascii_digits[0] == b'-';
    if is_neg {ascii_digits = &ascii_digits[1..];}
    let len = get_int_len(ascii_digits);
    if len > 0 { ascii_digits = &ascii_digits[..len];}
    let z = I::from_u64(0).unwrap();
    let l = I::from_u64(1).unwrap();
    let ten = I::from_u64(10).unwrap();
    let nr= ascii_digits.iter().rev().fold_while(
        (z,l), |(mut acc,mut pot) : (I,I),x : &u8|
        match atoi::<I>(*x) {
            Ok(int) => {        
                println!("acc is {} pot is {}",acc,pot);
                acc = acc+ pot * int;
                pot = pot * ten;
                return Continue((acc,pot))
            },
            _ => {
                println!("facc is {} pot is {} byte is {}",acc,pot,x);
                Done((acc,pot))
            }
        }
    ).into_inner();
    match nr.1 == l{
        true => None,
        false => Some( if is_neg {z-nr.0} else {nr.0})
    }
}


pub fn parse_int_from_ascii(mut ascii_digits : &[u8]) -> Result<i64,u8>{
    if ascii_digits.len() == 0 { return Err(0)};
    let is_neg : bool = ascii_digits[0] == b'-';
    if is_neg {ascii_digits = &ascii_digits[1..];}
    let nr= ascii_digits.iter().rev().try_fold(
        (0i64,1i64), |(mut acc,mut pot),x|
        match atoi::<i64>(*x) {
            Ok(int) => {        
                acc += pot * int;
                pot *=10;
                return Ok((acc,pot))
            },
            _ => Err(*x)
        }
    );
    match nr{
        Ok((x,_)) => Ok( if is_neg {-x} else {x}),
        Err(x) => Err(x)
    }
}




//impl parse, which parses a slice of bytes. Then call parse_between_sep to parse each between a sep.
pub trait Parser{
    type Out;
    type ErrorT;
    fn parse(bytes : &[u8]) -> Result<Self::Out,Self::ErrorT>;


    fn parse_and_add_to_vec(bytes : &[u8],vec : &mut Vec<Self::Out>) -> Result<(),Self::ErrorT>{
        let parsed_obj_mb = Self::parse(bytes);
        if let Ok(parsed_obj) = parsed_obj_mb  {
            vec.push(parsed_obj);
            Ok(())                        
        } else if let Err(x) = parsed_obj_mb{
           return Err(x);                       
        } else{
            panic!("this should not be possible")
        }
    }

    // parses everything that can be parsed. everything else is ignored
    fn parse_all_between_sep(bytes : &[u8],sep : u8) -> Result<Vec<Self::Out>,Self::ErrorT>{
        let mut lead_ind = 0;
        let mut res : Vec<Self::Out> = Vec::new();
        for (i,c) in bytes.iter().enumerate(){
            if *c == sep{
                let _ = Self::parse_and_add_to_vec(&bytes[lead_ind..i],&mut res);
                lead_ind = i + 1;
            }
        }
        let _ = Self::parse_and_add_to_vec(&bytes[lead_ind..],&mut res);
        Ok(res)
    }

    //parses until first error then returns
    fn parse_until_err_between_sep(bytes : &[u8],sep : u8) -> Result<Vec<Self::Out>,Self::ErrorT>{
        let mut lead_ind = 0;
        let mut res : Vec<Self::Out> = Vec::new();
        for (i,c) in bytes.iter().enumerate(){
            if *c == sep{
                if let Err(_) = Self::parse_and_add_to_vec(&bytes[lead_ind..i],&mut res) {return Ok(res)}
                lead_ind = i + 1;
            }
        }
        if let Err(_) = Self::parse_and_add_to_vec(&bytes[lead_ind..],&mut res) {return Ok(res)}
        Ok(res)
    }

    fn parse_until_err_strict(bytes : &[u8],sep : u8) -> Result<Vec<Self::Out>,Self::ErrorT>{
        let mut lead_ind = 0;
        let mut res : Vec<Self::Out> = Vec::new();
        for (i,c) in bytes.iter().enumerate(){
            if *c == sep{
                if let Err(x) = Self::parse_and_add_to_vec(&bytes[lead_ind..i],&mut res) {return Err(x)}
                lead_ind = i + 1;
            }
        }
        if let Err(x) = Self::parse_and_add_to_vec(&bytes[lead_ind..],&mut res) {return Err(x)}
        Ok(res)
    }

}


pub struct IntReader {}

impl Parser for IntReader{
    type Out = i64;
    type ErrorT = String;


    //ignore if empty string, otherwise 
    fn parse(bytes : &[u8]) -> Result<Self::Out,Self::ErrorT> {
        match parse_int_from_ascii(bytes){
            Ok(x) => Ok(x),
            Err(c) => Err(format!("found non-digit character {}",c))
        }
    }
}

#[cfg(debug_assertions)]
pub fn inp_path(puzzle_date : u64,nr : u64) -> String {format!("input/P{:02}/test{}.txt",puzzle_date,nr)}
#[cfg(not(debug_assertions))]
pub fn inp_path(puzzle_date : u64,nr : u64) -> String {format!("input/P{:02}/input.txt",puzzle_date)}

pub fn read_input_of_puzzle(puzzle_date : u64,puzzle_nr : u64) -> Result<Vec<u8>,io::Error>{
    let path = inp_path(puzzle_date,puzzle_nr);
    fs::read(Path::new(&path))
}


#[cfg(test)]
mod test_mod_file_read_utils {
    use super::*;
    
    #[test]
    fn test_parse_all_btw_sep() {
        let file = fs::read("test.txt").expect("could not find file");
        let parsed_vals_mb = IntReader::parse_all_between_sep(&file, b'\n');
        if let Ok(parsed_vals) = parsed_vals_mb{
            assert_eq!(parsed_vals[0],72595);
            assert_eq!(parsed_vals[1],139673);
            assert_eq!(parsed_vals[2],157);
        } else if let Err(err_str) = parsed_vals_mb{
            assert!(false,"{}",err_str);
        }
        
    }

    #[test]
    fn test_parse_till_err_btw_sep() {
        let file = fs::read("test.txt").expect("could not find file");
        let parsed_vals_mb = IntReader::parse_until_err_between_sep(&file, b'\n');
        if let Ok(parsed_vals) = parsed_vals_mb{
            assert_eq!(parsed_vals[0],72595);
            assert_eq!(parsed_vals[1],139673);
            assert_eq!(parsed_vals.len(),2);
        } else if let Err(err_str) = parsed_vals_mb{
            assert!(false,"{}",err_str);
        }
        
    }

    #[test]
    fn test_parse_int_greedy() {
        assert_eq!(parse_int_greedy("48a".as_bytes()),Some(48u64));
        assert_eq!(parse_int_greedy("69zq54a".as_bytes()),Some(69u64));
        assert_eq!(parse_int_greedy::<u64>("t69zq54a".as_bytes()),None);
    }


}

