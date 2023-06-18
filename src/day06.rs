use std::collections::HashSet;
use std::fs::read_to_string;

pub fn run(args: &[String]) -> std::io::Result<()> {
    let input: String = read_to_string("input/day06.txt")?;

    match args.get(0).map(|s| s.parse::<i32>()) {
        Some(Ok(1)) => find_start_of_distinct(input, 4),
        Some(Ok(2)) => find_start_of_distinct(input, 14),
        _ => panic!("Unknown part"),
    }?;

    Ok(())
}

fn find_start_of_distinct(input: String, nr_of_distinct: usize) -> std::io::Result<()> {
    if let Some((index, _)) =
        input
            .as_bytes()
            .windows(nr_of_distinct)
            .enumerate()
            .find(|(_, bytes)| {
                (*bytes).iter().copied().collect::<HashSet<u8>>().len() == nr_of_distinct
            })
    {
        println!(
            "First {} different characters start at index {}",
            nr_of_distinct,
            index + nr_of_distinct
        );
    }
    Ok(())
}
