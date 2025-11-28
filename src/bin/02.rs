advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    fn extract(str: &str) -> Vec<Vec<usize>> {
        let count = str.lines().count();
        let mut vecs = Vec::with_capacity(count);
        for line in str.lines() {
            let count = line.split_whitespace().count();
            let mut vec = Vec::with_capacity(count);
            for val in line.split_whitespace() {
                let val: usize = val.parse().unwrap();
                vec.push(val);
            }
            vecs.push(vec);
        }
        vecs
    }

    fn process_line(a: &[usize], increasing: bool) -> bool {
        for chunk in a.windows(2) {
            let d = if increasing {
                chunk[1] as i64 - chunk[0] as i64
            } else {
                chunk[0] as i64 - chunk[1] as i64
            };
            let ok = matches!(d, 1..=3);
            if !ok {
                return false;
            }
        }
        true
    }
    fn process(vecs: Vec<Vec<usize>>) -> usize {
        let mut out = 0;
        for a in vecs.iter() {
            let ok = process_line(a, true) || process_line(a, false);
            if ok {
                out += 1;
            }
        }
        out
    }

    let out = extract(input);
    let out = process(out);
    Some(out as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    fn extract(str: &str) -> Vec<Vec<usize>> {
        let count = str.lines().count();
        let mut vecs = Vec::with_capacity(count);
        for line in str.lines() {
            let count = line.split_whitespace().count();
            let mut vec = Vec::with_capacity(count);
            for val in line.split_whitespace() {
                let val: usize = val.parse().unwrap();
                vec.push(val);
            }
            vecs.push(vec);
        }
        vecs
    }

    fn process_line(a: &[usize], increasing: bool) -> bool {
        for chunk in a.windows(2) {
            let d = if increasing {
                chunk[1] as i64 - chunk[0] as i64
            } else {
                chunk[0] as i64 - chunk[1] as i64
            };
            let ok = matches!(d, 1..=3);
            if !ok {
                return false;
            }
        }
        true
    }
    fn process_line_at(a: &[usize], loc: usize) -> bool {
        let a: Vec<usize> = a
            .iter()
            .enumerate()
            .filter(|(i, _x)| i != &loc)
            .map(|(_i, x)| *x)
            .collect();
        process_line(&a, true) || process_line(&a, false)
    }
    fn process(vecs: Vec<Vec<usize>>) -> usize {
        let mut out = 0;
        for a in vecs.iter() {
            for i in 0..a.len() {
                let ok = process_line_at(a, i);
                if ok {
                    out += 1;
                    break;
                }
            }
        }
        out
    }
    let out = extract(input);
    let out = process(out);
    Some(out as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
