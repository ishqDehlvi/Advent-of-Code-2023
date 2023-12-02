use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn is_possible(game: &Vec<HashMap<String, usize>>, limits: &HashMap<String, usize>) -> bool {
    for subset in game {
        for (color, count) in subset {
            if let Some(&limit) = limits.get(color) {
                if *count > limit {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    true
}

fn main() {
    // Read input from file
    if let Ok(file) = File::open("src/day2.txt") {
        let reader = io::BufReader::new(file);
        let mut games: HashMap<usize, Vec<HashMap<String, usize>>> = HashMap::new();

        let mut game_count = 0;
        let mut line_count = 0; 

        for line in reader.lines() {
            line_count += 1; 
            let line = line.expect("Failed to read line");
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with("Game") {
                game_count += 1;
                games.insert(game_count, Vec::new());
            } else {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let mut game: HashMap<String, usize> = HashMap::new();

                for i in (0..parts.len()).step_by(2) {
                    let count: usize = match parts[i].parse() {
                        Ok(num) => num,
                        Err(_) => {
                            eprintln!("Error: Unable to parse integer from '{}'", parts[i]);
                            std::process::exit(1);
                        }
                    };
                    let color = parts[i + 1].to_string();
                    game.insert(color, count);
                }

                games.get_mut(&game_count).unwrap().push(game);
            }
        }

        println!("Number of lines read from the file: {}", line_count); 

        let limits: HashMap<String, usize> = [
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
        ]
        .iter()
        .cloned()
        .collect();

        let result = games
            .iter()
            .filter(|(_, game)| is_possible(game, &limits))
            .map(|(id, _)| *id) 
            .collect::<Vec<_>>();

        let sum: usize = result.iter().sum();
        println!("Sum of possible game IDs: {}", sum);
    } else {
        eprintln!("Error: Unable to open the file 'day2.txt'");
        std::process::exit(1);
    }
}
