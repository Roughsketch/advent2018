fn main() -> Result<(), std::io::Error> {
    for day in 1..=25 {
        base::run(day)?;
    }

    Ok(())
}
