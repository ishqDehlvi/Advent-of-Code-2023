pub(crate) fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    part1(&input);

    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    part2(&input);
}

fn part1(lines: &[String]) {
    let times: Vec<usize> = lines[0].split_whitespace().skip(1)
        .map(|s| s.parse::<usize>().unwrap()).collect();
    let distances: Vec<usize> = lines[1].split_whitespace().skip(1)
        .map(|s| s.parse::<usize>().unwrap()).collect();

    let total: usize = times.iter().zip(distances.iter())
        .map(|(time, distance)| simulation_count(*time, *distance))
        .product();

    println!("part1: {}", total);
}

fn simulation_count(time: usize, distance: usize) -> usize {
    (0..=time).filter(|&t| speed_distance(t, time, distance)).count()
}

fn speed_distance(t: usize, time: usize, distance: usize) -> bool {
    let speed = time - t;
    let distance_traveled = speed * t;
    distance_traveled > distance
}

fn part2(lines: &[String]) {
    let time = lines[0].replace(" ", "").replace("Time:", "")
        .parse::<usize>().unwrap();
    let distance = lines[1].replace(" ", "").replace("Distance:", "")
        .parse::<usize>().unwrap();

    println!("part2: {}", simulation_count(time, distance));
}
