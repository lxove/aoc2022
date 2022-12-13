use itertools::Itertools;
use std::collections::HashMap;

fn solve_a(input: &str) -> i32 {
    let scoreMap = HashMap::from([
        (("A", "X"),4),
        (("A", "Y"),8),
        (("A", "Z"),3),
        (("B", "X"),1),
        (("B", "Y"),5),
        (("B", "Z"),9),
        (("C", "X"),7),
        (("C", "Y"),2),
        (("C", "Z"),6)
    ]);

    solve(input, scoreMap)
}


fn solve_b(input: &str) -> i32 {
    let scoreMap = HashMap::from([
        (("A", "X"),3),
        (("A", "Y"),4),
        (("A", "Z"),8),
        (("B", "X"),1),
        (("B", "Y"),5),
        (("B", "Z"),9),
        (("C", "X"),2),
        (("C", "Y"),6),
        (("C", "Z"),7)
    ]);

    solve(input, scoreMap)
    
}

fn solve(input: &str, scoreMap:HashMap<(&str,&str), i32>) -> i32 {
    input.lines()
        .map(|line| {
            let tuple = line.split(" ").collect_tuple::<(&str, &str)>().unwrap();
            scoreMap.get(&tuple).unwrap()
        })
        .fold(0,|acc, current| acc + current)

}


#[cfg(test)]
mod tests {
    #[test]
    fn solve_a(){
            assert_eq!(super::solve_a(include_str!("input.txt")),11666)
        }
        
    #[test]
    fn solve_b(){
        assert_eq!(super::solve_b(include_str!("input.txt")),12767)
    }
}

