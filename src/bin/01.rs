advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    fn extract_lists(str: &str) -> [Vec<u32>; 2] {
        let count = str.lines().count();
        let mut vecs = [Vec::with_capacity(count), Vec::with_capacity(count)];
        for line in str.lines() {
            let (a, b) = line.split_once(" ").unwrap();
            let a: u32 = a.parse().unwrap();
            let b: u32 = b.trim().parse().unwrap();
            vecs[0].push(a);
            vecs[1].push(b);
        }
        vecs
    }

    fn process_lists(mut vecs: [Vec<u32>; 2]) -> u32 {
        vecs[0].sort();
        vecs[1].sort();
        let mut diff = 0;
        for (a, b) in vecs[0].iter().zip(vecs[1].iter()) {
            diff += a.abs_diff(*b);
        }
        diff
    }
    let test = extract_lists(input);
    let test = process_lists(test);
    Some(test.into())
}

pub fn part_two(input: &str) -> Option<u64> {
    use counter::Counter;

    fn extract_lists(str: &str) -> [Vec<usize>; 2] {
        let count = str.lines().count();
        let mut vecs = [Vec::with_capacity(count), Vec::with_capacity(count)];
        for line in str.lines() {
            let (a, b) = line.split_once(" ").unwrap();
            let a: usize = a.parse().unwrap();
            let b: usize = b.trim().parse().unwrap();
            vecs[0].push(a);
            vecs[1].push(b);
        }
        vecs
    }

    fn process_lists(vecs: [Vec<usize>; 2]) -> usize {
        let counter = vecs[1].iter().collect::<Counter<_>>();
        let mut out = 0;
        for a in vecs[0].iter() {
            out += a * counter[a];
        }
        out
    }
    let test = extract_lists(input);
    let test = process_lists(test);
    Some(test as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
