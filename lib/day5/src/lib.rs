fn react(mut content: String) -> String {
    let mut last_len = 0;

    //  While we're still combining characters
    while last_len != content.len() {
        //  Update the length for next iteration
        last_len = content.len();
        let bytes = content.as_bytes();

        //  Loop through content to find a combination
        for index in 0..content.len() - 1 {
            //  Look for two chars where one is uppercase and one is lowercase
            if bytes[index] as u8 == bytes[index + 1] as u8 ^ 32 {
                //  Cut out the 2 chars that combined
                let mut prefix = bytes[0..index].to_vec();
                prefix.extend(&bytes[index + 2..]);

                //  Re-assign content
                content = String::from_utf8(prefix).unwrap();
                break;
            }
        }
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
