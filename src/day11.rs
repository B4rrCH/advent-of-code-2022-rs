use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Lines, Result},
};

pub fn run(args: &[String]) -> std::io::Result<()> {
    let file = File::open("input/day11.txt")?;
    let reader = BufReader::new(file);

    let monkeys = parse_monkeys(reader.lines());

    match args
        .get(0)
        .ok_or(Error::from(ErrorKind::InvalidData))?
        .parse()
        .map_err(|err| Error::new(ErrorKind::Other, err))?
    {
        1 => part1(monkeys),
        2 => part2(monkeys),
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub struct Monkey {
    items: VecDeque<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test_divisible_by: usize,
    throw_to_if_true: usize,
    throw_to_if_false: usize,
    number_of_inspections: usize,
}

pub fn parse_monkeys<B: BufRead>(lines: Lines<B>) -> Vec<Monkey> {
    let mut lines = lines;
    let mut monkeys = Vec::new();
    while let Some(monkey) = parse_monkey(&mut lines) {
        monkeys.push(monkey);
    }
    monkeys
}

pub fn parse_monkey<B: BufRead>(lines: &mut Lines<B>) -> Option<Monkey> {
    _ = lines.next();

    let items = {
        let items_str = lines.next().and_then(Result::ok)?["  Starting items: ".len()..].to_owned();
        let mut items = VecDeque::new();
        for item_str in items_str.split(", ") {
            items.push_back(item_str.parse().ok()?);
        }
        Some(items)
    }?;
    let operation = parse_operation(lines)?;
    let test_divisible_by = parse_number_after(lines, "  Test: divisible by ")?;
    let throw_to_if_true = parse_number_after(lines, "    If true: throw to monkey ")?;
    let throw_to_if_false = parse_number_after(lines, "    If false: throw to monkey ")?;

    _ = lines.next();

    Some(Monkey {
        items,
        operation,
        test_divisible_by,
        throw_to_if_true,
        throw_to_if_false,
        number_of_inspections: 0,
    })
}

fn parse_number_after<B: BufRead>(reader: &mut Lines<B>, s: &str) -> Option<usize> {
    reader.next().and_then(Result::ok)?[s.len()..].parse().ok()
}

fn parse_operation<B: BufRead>(reader: &mut Lines<B>) -> Option<Box<dyn Fn(usize) -> usize>> {
    let operation_str =
        reader.next().and_then(Result::ok)?["  Operation: new = old ".len()..].to_owned();

    let (op, arg) = operation_str.split_once(' ')?;
    match (op, arg.parse::<usize>().ok()) {
        ("+", Some(n)) => Some(Box::new(move |x| x + n)),
        ("*", Some(n)) => Some(Box::new(move |x| x * n)),
        ("+", None) => Some(Box::new(|x| x + x)),
        ("*", None) => Some(Box::new(|x| x * x)),
        _ => None,
    }
}

fn part1(monkeys: Vec<Monkey>) -> Result<()> {
    const NUMBER_OF_ROUNDS: i32 = 20;
    let mut monkeys = monkeys;

    for _ in 0..NUMBER_OF_ROUNDS {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].number_of_inspections += 1;

                let after_inspection = (monkeys[i].operation)(item) / 3;

                let throw_to = if after_inspection % monkeys[i].test_divisible_by == 0 {
                    monkeys[i].throw_to_if_true
                } else {
                    monkeys[i].throw_to_if_false
                };

                monkeys[throw_to].items.push_back(after_inspection);
            }
        }
    }

    for (i, m) in monkeys.iter().enumerate() {
        println!(
            "Monkey {} inspected items {} times.",
            i, m.number_of_inspections
        );
    }

    let (x, y) = find_top_two(monkeys.iter().map(|m| m.number_of_inspections));
    println!("Worry level is: {}", x * y);

    Ok(())
}

fn part2(monkeys: Vec<Monkey>) -> std::io::Result<()> {
    const NUMBER_OF_ROUNDS: i32 = 10_000;
    let mut monkeys = monkeys;

    let product_of_divisibility_tests: usize =
        monkeys.iter().map(|m| m.test_divisible_by).product();

    for _ in 0..NUMBER_OF_ROUNDS {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].number_of_inspections += 1;

                let after_inspection = (monkeys[i].operation)(item) % product_of_divisibility_tests;

                let throw_to = if after_inspection % monkeys[i].test_divisible_by == 0 {
                    monkeys[i].throw_to_if_true
                } else {
                    monkeys[i].throw_to_if_false
                };

                monkeys[throw_to].items.push_back(after_inspection);
            }
        }
    }

    for (i, m) in monkeys.iter().enumerate() {
        println!(
            "Monkey {} inspected items {} times.",
            i, m.number_of_inspections
        );
    }

    let (x, y) = find_top_two(monkeys.iter().map(|m| m.number_of_inspections));
    println!("Worry level is: {}", x * y);

    Ok(())
}

fn find_top_two<I>(vals: I) -> (usize, usize)
where
    I: Iterator<Item = usize>,
{
    let mut top_two = BinaryHeap::new();
    for _ in 0..2 {
        top_two.push(Reverse(0));
    }

    for val in vals {
        let Reverse(min) = top_two.peek().unwrap();
        if *min < val {
            top_two.pop();
            top_two.push(Reverse(val));
        }
    }

    (top_two.pop().unwrap().0, top_two.pop().unwrap().0)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::day11::*;
    use std::{
        io::{BufReader, Read},
        slice::Iter,
    };

    use super::parse_monkey;

    struct StringReader<'a> {
        iter: Iter<'a, u8>,
    }

    impl<'a> StringReader<'a> {
        pub fn new(data: &'a str) -> Self {
            Self {
                iter: data.as_bytes().iter(),
            }
        }
    }

    impl<'a> Read for StringReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            for i in 0..buf.len() {
                if let Some(x) = self.iter.next() {
                    buf[i] = *x;
                } else {
                    return Ok(i);
                }
            }
            Ok(buf.len())
        }
    }

    #[test]
    fn parse_monkey_of_monkey_string_with_addition_should_work() {
        // Arrange
        let monkey_string = concat!(
            "Monkey 1:\n",
            "  Starting items: 2, 3, 4\n",
            "  Operation: new = old + 5\n",
            "  Test: divisible by 6\n",
            "    If true: throw to monkey 7\n",
            "    If false: throw to monkey 8\n"
        );
        let reader = BufReader::new(StringReader::new(monkey_string));

        // Act
        let monkey = parse_monkey(&mut reader.lines());

        // Assert
        let monkey = monkey.unwrap();
        assert_eq!(vec![2, 3, 4], monkey.items.into_iter().collect_vec());

        let op = monkey.operation;

        for i in 0..100 {
            assert_eq!(i + 5, op(i));
        }

        assert_eq!(6, monkey.test_divisible_by);
        assert_eq!(7, monkey.throw_to_if_true);
        assert_eq!(8, monkey.throw_to_if_false);
    }

    #[test]
    fn parse_monkey_of_monkey_string_with_multiplication_should_work() {
        // Arrange
        let monkey_string = concat!(
            "Monkey 0:\n",
            "  Starting items: 85, 77, 77\n",
            "  Operation: new = old * 7\n",
            "  Test: divisible by 19\n",
            "    If true: throw to monkey 6\n",
            "    If false: throw to monkey 7\n"
        );
        let reader = BufReader::new(StringReader::new(monkey_string));

        // Act
        let monkey = parse_monkey(&mut reader.lines());

        // Assert
        let monkey = monkey.unwrap();
        assert_eq!(vec![85, 77, 77], monkey.items.into_iter().collect_vec());

        let op = monkey.operation;

        for i in 0..100 {
            assert_eq!(i * 7, op(i));
        }

        assert_eq!(19, monkey.test_divisible_by);
    }

    #[test]
    fn parse_monkey_of_monkey_string_with_squaring_should_work() {
        // Arrange
        let monkey_string = concat!(
            "Monkey 0:\n",
            "  Starting items: 85, 77, 77\n",
            "  Operation: new = old * old\n",
            "  Test: divisible by 19\n",
            "    If true: throw to monkey 6\n",
            "    If false: throw to monkey 7\n"
        );
        let reader = BufReader::new(StringReader::new(monkey_string));

        // Act
        let monkey = parse_monkey(&mut reader.lines());

        // Assert
        let monkey = monkey.unwrap();
        assert_eq!(vec![85, 77, 77], monkey.items.into_iter().collect_vec());

        let op = monkey.operation;
        for i in 0..100 {
            assert_eq!(i * i, op(i));
        }

        assert_eq!(19, monkey.test_divisible_by);
    }
}
