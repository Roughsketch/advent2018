pub fn part_1() -> Result<i32, std::io::Error> {
    let factors = std::fs::read_to_string("input/2.txt")?
        .split_whitespace()
        .fold((0, 0), |mut factors, s| {
            //  Keep map to make histogram for char frequency
            let mut map = std::collections::HashMap::new();

            //  For every char, insert or update value in histogram
            for c in s.chars() {
                let entry = map.entry(c).or_insert(0);
                *entry += 1;
            }

            //  For every value with a count of exactly 2, add 1 to factor 1
            if map.iter().filter(|(_, v)| **v == 2).count() > 0 {
                factors.0 += 1;
            }

            //  For every value with a count of exactly 3, add 1 to factor 2
            if map.iter().filter(|(_, v)| **v == 3).count() > 0 {
                factors.1 += 1;
            }

            factors
        });

    //  Answer is both factors multiplied to make checksum
    Ok(factors.0 * factors.1)
}

pub fn part_2() -> Result<String, std::io::Error> {
    let contents = std::fs::read_to_string("input/2.txt")?;
    let factors = contents.split_whitespace().collect::<Vec<&str>>();

    for (current, factor) in factors.iter().enumerate() {
        //  For each factor, loop over subsequent factors to check
        for index in current..factors.len() {
            let mut diff = 0;   //  Number of differences
            let mut common = String::new(); //  String of common characters

            for (c1, c2) in factor.chars().zip(factors.get(index).unwrap().chars()) {
                //  If chars are different, add to diff
                if c1 != c2 {
                    diff += 1;
                } else {
                    //  If they are identical, add to common list
                    common.push(c1);
                }

                //  If there were 2 differences, there is no point in checking further
                if diff > 1 {
                    break;
                }
            }

            //  If there was only 1 difference, then this is the answer
            if diff == 1 {
                return Ok(common);
            }
        }
    }

    //  Return empty string if not found
    Ok(String::new())
}