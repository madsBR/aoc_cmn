pub mod file_read_utils;
pub mod solution;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn subtr(l : usize) -> usize{
    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

