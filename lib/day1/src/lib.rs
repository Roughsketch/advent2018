pub fn part_1() -> Result<i32, std::io::Error> {
    Ok(std::fs::read_to_string("input/1.txt")?      //  Read contents
        .split_whitespace()                         //  Split into lines
        .filter_map(|line| line.parse::<i32>().ok())//  Parse each line as a number
        .sum())                                     //  And sum
}

pub fn part_2() -> Result<i32, std::io::Error> {
    //  Parse file into a vector of i32
    let changes = std::fs::read_to_string("input/1.txt")?
        .split_whitespace()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let mut set = std::collections::HashSet::new();
    let mut freq = 0;

    //  Continually apply frequency changes
    'outer: loop {
        for change in &changes {
            freq += change;

            //  If frequency already exists in the set, then break
            if !set.insert(freq) {
                break 'outer;
            }
        }
    }

    Ok(freq)
}
