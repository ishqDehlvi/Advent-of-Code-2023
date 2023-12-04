use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};

fn parse_card(line: &str) -> (HashSet<&str>, Vec<&str>) {
    let parts: Vec<&str> = line.split(":").collect();
    let winning_numbers = parts[1].split("|").next().unwrap().split_whitespace().collect();
    let numbers_given = parts[1].split("|").nth(1).unwrap().split_whitespace().collect();

    (winning_numbers, numbers_given)
}

fn count_wins() -> usize {
    let mut points_total = 0;
    let mut card_points = HashMap::new();

    if let Ok(file) = File::open("src/input.txt") {
        let reader = io::BufReader::new(file);

        for (idx, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                let (winning_numbers, numbers_given) = parse_card(&line.trim());

                let points: usize = numbers_given.iter().filter(|&&number| winning_numbers.contains(number)).count();
                card_points.insert(idx + 1, points);
            }
        }
    }

    for card in card_points.keys() {
        points_total += count_number_scratch_cards(&card_points, *card, &mut HashMap::new());
    }

    points_total
}

fn count_number_scratch_cards(card_points: &HashMap<usize, usize>, current_card: usize, memo: &mut HashMap<usize, usize>) -> usize {
    let mut total = 1;
    let mut i = current_card;

    while card_points.contains_key(&current_card) && i < card_points[&current_card] + current_card {
        i += 1;

        if !memo.contains_key(&i) {
            total += count_number_scratch_cards(card_points, i, memo);
        } else {
            total += memo[&i];
        }
    }

    memo.insert(current_card, total);
    total
}

fn main() {
    println!("{}", count_wins());
}
