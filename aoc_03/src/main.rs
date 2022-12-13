use std::collections::HashSet;

fn solve_a(input: &str) -> u32 {
    let mut sum: u32 = 0;

    input.lines()
    .for_each(|line| {
        let (a,b) = line.split_at(line.len()/2);
        let a = a.chars().collect::<HashSet<char>>();
        let b = b.chars().collect::<HashSet<char>>();

        let mut character = a.intersection(&b).collect::<HashSet<_>>();
        
        for &i in character.drain() {
            let mut val = i as u32;
            if i.is_lowercase() {
                val = val - 96;
            } else {
                val = val - 38;
            }
            sum += val;
        }


    });
    sum
}

#[cfg(test)]

mod tests {
    #[test]
    fn solve_a(){
        assert_eq!(super::solve_a(include_str!("input.txt")), 7795)
    }
}
