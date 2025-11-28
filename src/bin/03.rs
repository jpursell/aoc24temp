use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    #[derive(Debug)]
    struct Mul {
        a: usize,
        b: usize,
    }
    fn extract(str: &str) -> Vec<Mul> {
        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        let mut out = Vec::new();
        for (_, [a, b]) in re.captures_iter(str).map(|c| c.extract()) {
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            out.push(Mul { a, b });
        }
        out
    }

    fn process(vecs: Vec<Mul>) -> usize {
        let mut out = 0;
        for a in vecs.iter() {
            out += a.a * a.b;
        }
        out
    }

    let out = extract(input);
    let out = process(out);
    // assert!(out > 1552138);
    // println!("{out:?}");
    Some(out as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    #[derive(Clone, Copy, Debug)]
    enum Token {
        Do,
        Dont,
        Mul(usize),
        None,
    }
    fn extract(str: &str) -> Vec<Token> {
        let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        let do_re = Regex::new(r"do\(\)").unwrap();
        let dont_re = Regex::new(r"don't\(\)").unwrap();

        let mut tokens = vec![Token::None; str.len()];
        for cap in mul_re.captures_iter(str) {
            let loc = cap.get(0).unwrap().start();
            let (_, [a, b]) = cap.extract();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();
            let val = a * b;
            tokens[loc] = Token::Mul(val);
        }
        for cap in do_re.captures_iter(str) {
            let loc = cap.get(0).unwrap().start();
            tokens[loc] = Token::Do;
        }
        for cap in dont_re.captures_iter(str) {
            let loc = cap.get(0).unwrap().start();
            tokens[loc] = Token::Dont;
        }
        tokens
    }

    fn process(tokens: &[Token]) -> usize {
        let mut out = 0;
        let mut active = true;
        for token in tokens {
            match token {
                Token::Do => {
                    active = true;
                }
                Token::Dont => {
                    active = false;
                }
                Token::Mul(val) => {
                    if active {
                        out += val
                    }
                }
                _ => (),
            }
        }
        out
    }

    let out = extract(input);
    let out = process(&out);
    Some(out as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
