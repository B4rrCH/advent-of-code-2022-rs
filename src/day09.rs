use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Movement {
    direction: Direction,
    count: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

struct Rope {
    position: Position,
    rest: Option<Box<Rope>>,
}

impl Rope {
    fn make_step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.position.x -= 1,
            Direction::Down => self.position.x += 1,
            Direction::Left => self.position.y -= 1,
            Direction::Right => self.position.y += 1,
        };

        let mut head = &self.position;
        let mut rope = &mut self.rest;

        while rope.is_some() {
            let tmp_rope = rope.as_mut().unwrap();

            if !tmp_rope.position.move_towards(head) {
                return;
            }

            head = &tmp_rope.position;
            rope = &mut tmp_rope.rest;
        }
    }

    fn get_tail(&self) -> Position {
        let mut rope = self;
        while let Some(rest) = &rope.rest {
            rope = rest;
        }
        rope.position
    }
}

impl Position {
    fn move_towards(&mut self, position: &Position) -> bool {
        let dx = position.x - self.x;
        let dy = position.y - self.y;

        match max(dx.abs(), dy.abs()) {
            0 | 1 => false,
            _ => {
                self.x += dx.signum();
                self.y += dy.signum();
                true
            }
        }
    }
}

impl TryFrom<String> for Movement {
    type Error = std::io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.split_once(' ') {
            Some((symbol, number)) => {
                let direction = match symbol {
                    "U" => Ok(Direction::Up),
                    "D" => Ok(Direction::Down),
                    "L" => Ok(Direction::Left),
                    "R" => Ok(Direction::Right),
                    _ => Err(Error::from(ErrorKind::InvalidInput)),
                }?;
                let count = number
                    .parse::<usize>()
                    .map_err(|err| Error::new(ErrorKind::Other, err))?;

                Ok(Movement { direction, count })
            }

            _ => Err(Error::from(ErrorKind::InvalidInput)),
        }
    }
}

pub fn run(args: &[String]) -> std::io::Result<()> {
    let file = File::open("input/day09.txt")?;
    let reader = BufReader::new(file);

    let pieces_of_rope: usize = args
        .get(0)
        .ok_or(Error::from(ErrorKind::InvalidData))?
        .parse()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    let mut positions = Rope {
        position: Position { x: 0, y: 0 },
        rest: None,
    };

    for _ in 0..(pieces_of_rope - 1) {
        positions = Rope {
            position: Position { x: 0, y: 0 },
            rest: Some(Box::new(positions)),
        }
    }

    let mut seen = HashSet::new();

    seen.insert(Position { x: 0, y: 0 });
    for line in reader.lines() {
        let Movement { direction, count } = line?.try_into()?;
        for _ in 0..count {
            positions.make_step(&direction);
            seen.insert(positions.get_tail());
        }
    }

    println!("Tail was at {} positions", seen.len());

    Ok(())
}
