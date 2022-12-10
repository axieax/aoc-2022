pub fn part_one(input: &str) -> Option<u32> {
    let contents = String::from(input) + "\n";
    let mut max = 0;
    let mut curr_calories = 0;
    for line in contents.lines() {
        if line.is_empty() {
            max = std::cmp::max(max, curr_calories);
            curr_calories = 0;
        } else {
            curr_calories += line.parse::<u32>().unwrap();
        }
    }
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let contents = input.trim_end();
    let mut elf_calories = contents.split("\n\n").map(|elf| {
        elf.split("\n").map(|line| {
            line.parse::<u32>().unwrap()
        }).sum::<u32>()
    }).collect::<Vec<u32>>();

    // sort elf_calories in descending order
    elf_calories.sort_by(|a, b| b.cmp(a));
    Some(elf_calories.iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
