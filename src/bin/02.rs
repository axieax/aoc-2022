fn score_one(opponent_move: &str, my_move: &str) -> u32 {
    // calculate winner
    let score = match (opponent_move, my_move) {
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        _ => 0,
    };
    // calculate move score
    match my_move {
        "X" => score + 1,
        "Y" => score + 2,
        "Z" => score + 3,
        _ => panic!("Invalid my_move {}", my_move),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rows: Vec<u32> = input.lines().map(|round| {
        let moves = round.split(" ").collect::<Vec<&str>>();
        score_one(moves[0], moves[1])
    }).collect();
    Some(rows.iter().sum())
}

fn score_two(opponent_move: &str, outcome: &str) -> u32 {
    // calculate outcome
    let score = match outcome {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("Invalid outcome {}", outcome),
    };
    // calculate my_move score
    match (opponent_move, outcome) {
        // my_move: rock
        // win if opp scissors, tie if opp rock, lose if opp paper
        ("C", "Z") | ("A", "Y") | ("B", "X") => score + 1,
        // my_move: paper
        // win if opp rock, tie if opp paper, lose if opp scissors
        ("A", "Z") | ("B", "Y") | ("C", "X") => score + 2,
        // my_move: scissors
        // win if opp paper, tie if opp scissors, lose if opp rock
        ("B", "Z") | ("C", "Y") | ("A", "X") => score + 3,
        _ => panic!("Invalid opponent move {} and outcome {}", opponent_move, outcome),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows: Vec<u32> = input.lines().map(|round| {
        let moves = round.split(" ").collect::<Vec<&str>>();
        score_two(moves[0], moves[1])
    }).collect();
    Some(rows.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
