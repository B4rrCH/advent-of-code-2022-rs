use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

pub fn run(args: &[String]) -> std::io::Result<()> {
    let mut file = File::open("input/day06.txt")?;
    let mut input = vec![];
    file.read_to_end(&mut input)?;

    match args.get(0).map(|s| s.parse::<i32>()) {
        Some(Ok(1)) => find_start_of_distinct(&input, 4),
        Some(Ok(2)) => find_start_of_distinct(&input, 14),
        _ => panic!("Unknown part"),
    }?;

    Ok(())
}

fn find_start_of_distinct(input: &[u8], nr_of_distinct: usize) -> std::io::Result<()> {
    if let Some((index, _)) = input
        .windows(nr_of_distinct)
        .enumerate()
        .find(|(_, bytes)| (*bytes).iter().copied().collect::<HashSet<_>>().len() == nr_of_distinct)
    {
        println!(
            "First {} different characters start at index {}",
            nr_of_distinct,
            index + nr_of_distinct
        );
    }
    Ok(())
}
