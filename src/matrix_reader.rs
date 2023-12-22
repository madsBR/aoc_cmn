use std::io::BufRead;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::Neg;
use std::process::Output;

use crate::file_read_utils::get_buf_reader;


pub const O : Pos = Pos{ x : 0, y : 0};
pub const Ux : Pos = Pos{ x : 1, y : 0};
pub const Uy : Pos = Pos{ x : 0, y : 1};

//contains a vector of data, along with number of rows and number of cols
#[derive(Debug)]
pub struct Matrix{
    pub n_r : u64,
    pub n_c : u64,
    data : Vec<u8>
}

pub trait Position : Sized{
    
    fn to_index(&self, m: &Matrix) -> usize;
    fn from_index(index : usize, m: &Matrix) -> Self;
    fn at_north_border(&self, m: &Matrix) -> bool{
        let i =  self.to_index(m);
        i < m.n_c as usize            
    }
    fn at_south_border(&self, m: &Matrix) -> bool{
        let i =  self.to_index(m);
        i >= m.n_c as usize * (m.n_r-1)  as usize            
    }
    fn at_west_border(&self, m: &Matrix) -> bool{
        let i =  self.to_index(m);
        i % m.n_c as usize == 0    
    }
    fn at_east_border(&self, m: &Matrix) -> bool{
        let i =  self.to_index(m);
        (i + m.n_c as usize - 1) % m.n_c as usize == 0    
    }

    fn origo(m : &Matrix) -> Self{
        Self::from_index(0, &m)
    }
    fn at_border(&self,m: &Matrix) ->bool{
        self.at_north_border(m) ||
        self.at_south_border(m) ||
        self.at_west_border(m) ||
        self.at_east_border(m)
    }

}

impl Position for usize{
    fn to_index(&self,m : &Matrix) -> usize {
        *self
    }
    fn from_index(index : usize, m: &Matrix) -> Self {
        index
    }

}



//Origo is upper left corner, so that 0,0 has index 0)
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos{
    x: u64,
    y: u64
}
impl Position for Pos{
    #[inline(always)]
    fn to_index(&self, m: &Matrix) -> usize {
        (m.n_c * self.y + self.x) as usize                
    }

    #[inline(always)]
    fn from_index(index : usize,m: &Matrix) -> Pos{
         Pos{ x: (index as u64) % m.n_c, y : (index as u64) / m.n_r }
    }

    
   
}



impl Add<Pos> for Pos{
    type Output = Self;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Pos> for Pos{
    type Output = Self;
    fn sub(self, rhs: Pos) -> Self::Output {
        Pos{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}


impl AddAssign<Pos> for Pos{
    fn add_assign(&mut self, rhs: Pos) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
#[derive(Clone,Copy,PartialEq,Eq,PartialOrd, Ord,Debug)]
pub enum Dir{
    N,
    W,
    S,
    E
}
impl Neg for Dir{
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::N => Self::S,
            Self::E => Self::W,
            Self::S => Self::N,
            Self::W => Self::E,
        }
    }
}


impl Matrix{
    pub fn get_matrix_input(puzzle_date : u64,puzzle_nr : u64) -> Matrix{
        let mut vec = Vec::new();
        let mut r = get_buf_reader(puzzle_date, puzzle_nr);
        let nr = r.read_until(b'\n', &mut vec).expect("something went wrong reading from inp file.");
        let n_c = (nr-1) as u64;
        vec.pop();
        let mut n_r = 1;
        while r.read_until(b'\n', &mut vec).expect("something went wrong reading from inp file.") == nr{
            vec.pop();
            n_r +=1;
        }
        Matrix { n_r: n_r, n_c: n_c, data: vec }
    }


    
    #[inline(always)]
    pub fn data(&self) -> &Vec<u8>{ &self.data}
   #[inline(always)]
   pub fn data_mut(&mut self) -> &Vec<u8>{ &mut self.data}

   #[inline(always)]
   pub fn mv_index_in_dir(&self,i : usize, d : &Dir) -> Option<usize>{
        match d{
            Dir::N if self.index_to_pos(i).y == 0 => None,
            Dir::N => Some(i - self.n_c as usize),
            Dir::S if self.index_to_pos(i).y== (self.n_r-1) => None,
            Dir::S => Some(i + self.n_c as usize),
            Dir::W if self.index_to_pos(i).x == 0 => None ,
            Dir::W => Some(i - 1),
            Dir::E if self.index_to_pos(i).x == (self.n_c-1)=> None,
            Dir::E => Some(i + 1),

        }
   }




}