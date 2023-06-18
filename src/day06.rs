use std::fs::File;
use std::io::BufReader;

pub fn run(args: &[String]) -> std::io::Result<()> {
    let file = File::open("input/day06.txt")?;
    let reader = BufReader::new(file);

    match args.get(0).map(|s| s.parse::<i32>()) {
        Some(Ok(1)) => run_part1(reader),
        Some(Ok(2)) => run_part2(reader),
        _ => {
            println!("Unknown part");
            Ok(())
        }
    }?;

    Ok(())
}

fn run_part1(reader: BufReader<File>) -> std::io::Result<()> {
    _ = reader;
    Ok(())
}

fn run_part2(reader: BufReader<File>) -> std::io::Result<()> {
    _ = reader;
    Ok(())
}
