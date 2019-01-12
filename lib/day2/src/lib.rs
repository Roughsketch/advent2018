pub fn part_1() -> Result<i32, std::io::Error> {
    let factors = std::fs::read_to_string("input/2.txt")?
        .split_whitespace()
        .fold((0, 0), |mut factors, s| {
            let mut map = std::collections::HashMap::new();

            for c in s.chars() {
                let entry = map.entry(c).or_insert(0);
                *entry += 1;
            }

            if map.iter().filter(|(_, v)| **v == 2).count() > 0 {
                factors.0 += 1;
            }

            if map.iter().filter(|(_, v)| **v == 3).count() > 0 {
                factors.1 += 1;
            }

            factors
        });

    Ok(factors.0 * factors.1)
}

pub fn part_2() -> Result<String, std::io::Error> {
    let contents = std::fs::read_to_string("input/2.txt")?;
    let factors = contents.split_whitespace().collect::<Vec<&str>>();

    for (current, factor) in factors.iter().enumerate() {
        for index in current..factors.len() {
            let mut diff = 0;
            let mut common = String::new();

            for (c1, c2) in factor.chars().zip(factors.get(index).unwrap().chars()) {
                if c1 != c2 {
                    diff += 1;
                } else {
                    common.push(c1);
                }

                if diff > 1 {
                    break;
                }
            }

            if diff == 1 {
                return Ok(common);
            }
        }
    }

    Ok(String::new())
}