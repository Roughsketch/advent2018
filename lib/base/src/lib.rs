pub fn run(day: u32) -> Result<(), std::io::Error> {
    if day == 0 || day > 25 {
        return Ok(());
    }

    print!("Day {}: ", day);

    match day {
        1 => println!("{}, {}", day1::part_1()?, day1::part_2()?),
        2 => println!("{}, {}", day2::part_1()?, day2::part_2()?),
        3 => println!("{}, {}", day3::part_1()?, day3::part_2()?),
        4 => println!("{}, {}", day4::part_1()?, day4::part_2()?),
        5 => println!("{}, {}", day5::part_1()?, day5::part_2()?),
        _ => println!("Invalid day."),
    }

    Ok(())
}
