fn solve_a(input: &str) -> i32{
  42
}


fn solve_b(input: &str) -> i32 {
 42
}


#[cfg(test)]
mod tests{
    #[test]
    fn solve_a(){
            assert_eq!(super::solve_a(include_str!("input.txt")),0)
        }
        
    #[test]
    fn solve_b(){
        assert_eq!(super::solve_b(include_str!("input.txt")),0)
    }
  

}

