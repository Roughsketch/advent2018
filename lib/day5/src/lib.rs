fn react(mut content: String) -> String {
    let mut last_len = 0;

    //  While we're still combining characters
    while last_len != content.len() {
        //  Update the length for next iteration
        last_len = content.len();

        (b'a'..=b'z').for_each(|ch: u8| {
            let mut pair = String::new();
            let mut rev_pair = String::new();

            //  Create "xX" type string
            pair.push(ch as char);
            pair.push((ch ^ 32) as char);

            //  Create "Xx" type string
            rev_pair.push((ch ^ 32) as char);
            rev_pair.push(ch as char);

            content = content.replace(&pair, "")
                .replace(&rev_pair, "");
        });
    }

    content
}

pub fn part_1() -> Result<usize, std::io::Error> {
    let content = std::fs::read_to_string("input/5.txt")?;
    Ok(react(content).len())
}

pub fn part_2() -> Result<usize, std::io::Error> {
    let content = std::fs::read_to_string("input/5.txt")?;

    Ok((b'a'..=b'z')
        .map(|letter| {
            let rep = content
                .replace(letter as char, "")
                .replace((letter as u8 ^ 32) as char, "");

            react(rep).len()
        })
        .min()
        .unwrap_or(0))
}
