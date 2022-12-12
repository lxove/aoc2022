use std::cmp;


fn solve_a(input: &str) -> i32{
   let mut max:i32 = 0;
   let mut current:i32 = 0;

   for line in input.lines() {
    if line != "" {
        let parsed = line.parse::<i32>().unwrap();
        current += parsed;
    } else {
        max = cmp::max(current,max);
        current = 0;
    }
   }
   
   max
}

fn solve_a_mod(input: &str) -> i32 {
    let tuple: (i32,i32) = input
        .lines()
        .map(|line| {
            return match line.parse::<i32>() {
                Ok(s) => (s,s),
                Err(_) => (-1, -1)
            };
        })
        // fold funkar som js reduce, här passar vi (0,0) som startvärde
        .fold((0,0),|acc:(i32,i32), current:(i32,i32)| {
            if current.0 < 0 { (cmp::max(acc.0, acc.1), 0) } else { (acc.0, acc.1 + current.0) }
        });

    return cmp::max(tuple.0, tuple.1);
}


fn solve_b(input: &str) -> i32 {
   let mut max: [i32; 3] = [0,0,0];
   let mut current:i32 = 0;

   for line in input.lines() {
    if line != "" {
        let parsed = line.parse::<i32>().unwrap();
        current += parsed;
    } else {
        if current > max[0] {
          max = [current, max[0], max[1]];
        } else if current > max [1] {
            max = [max[0], current, max[1]];
        } else if current > max[2] {
            max = [max[0], max[1], current];
        } 
        current = 0;
    }
}

   max
    .into_iter()
    .reduce(|a,b| a+b)
    .unwrap()
}


#[cfg(test)]
mod tests{
    #[test]
    fn solve_a(){
            assert_eq!(super::solve_a(include_str!("input.txt")),70374)
        }
        
    #[test]
    fn solve_b(){
        assert_eq!(super::solve_b(include_str!("input.txt")),0)
    }
    
    #[test]
    fn solve_a_mod(){
    assert_eq!(super::solve_a_mod(include_str!("input.txt")),super::solve_a(include_str!("input.txt")))
    }

}

