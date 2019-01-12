pub fn part_1() -> Result<i32, std::io::Error> {
    Ok(std::fs::read_to_string("input/1.txt")?
        .split_whitespace()
        .filter_map(|line| line.parse::<i32>().ok())
        .sum())
}

pub fn part_2() -> Result<i32, std::io::Error> {
    let changes = std::fs::read_to_string("input/1.txt")?
        .split_whitespace()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let mut set = std::collections::HashSet::new();
    let mut freq = 0;

    'outer: loop {
        for change in &changes {
            freq += change;

            if !set.insert(freq) {
                break 'outer;
            }
        }
    }

    Ok(freq)
}
