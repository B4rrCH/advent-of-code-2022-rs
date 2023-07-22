use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Lines;

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    Add(i32),
}

impl TryFrom<String> for Instruction {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "noop" {
            return Ok(Self::Noop);
        }
        let (maybe_addx, number) = value
            .split_once(' ')
            .ok_or(Error::from(ErrorKind::InvalidData))?;
        if maybe_addx != "addx" {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        let x = number
            .parse::<i32>()
            .map_err(|parse_error| Error::new(ErrorKind::InvalidData, parse_error))?;
        Ok(Self::Add(x))
    }
}

pub fn run(args: &[String]) -> std::io::Result<()> {
    let file = File::open("input/day10.txt")?;
    let reader = BufReader::new(file);

    match args
        .get(0)
        .ok_or(Error::from(ErrorKind::InvalidData))?
        .parse()
        .map_err(|err| Error::new(ErrorKind::Other, err))?
    {
        1 => part1(reader),
        2 => part2(reader),
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

fn part1(reader: BufReader<File>) -> std::io::Result<()> {
    let mut cycle = 0;
    let mut register_x = 1;
    let mut sum_of_signal_strenghts = 0;

    for line in reader.lines() {
        let instruction: Instruction = line?.try_into()?;

        match instruction {
            Instruction::Noop => {
                cycle += 1;
                if cycle % 40 == 20 {
                    sum_of_signal_strenghts += cycle * register_x;
                }
            }
            Instruction::Add(augend) => {
                cycle += 1;
                if cycle % 40 == 20 {
                    sum_of_signal_strenghts += cycle * register_x;
                }
                cycle += 1;
                if cycle % 40 == 20 {
                    sum_of_signal_strenghts += cycle * register_x;
                }
                register_x += augend;
            }
        }
    }
    println!("Sum of signal strenghts is {}", sum_of_signal_strenghts);
    Ok(())
}

const DISPLAY_WIDTH: usize = 40;
const DISPLAY_HEIGHT: usize = 6;
struct Sprite {
    lines: Lines<BufReader<File>>,
    next_add: Option<i32>,
    register_x: i32,
}

impl Sprite {
    fn try_tick(&mut self) -> std::io::Result<bool> {
        if let Some(add) = self.next_add {
            self.register_x += add;
            self.next_add = None;
            return Ok(true);
        }

        match self.lines.next() {
            Some(Ok(x)) => {
                if let Instruction::Add(add) = x.try_into()? {
                    self.next_add = Some(add);
                } else {
                    self.next_add = None;
                }
                Ok(true)
            }
            Some(Err(err)) => Err(Error::new(ErrorKind::Other, err)),
            None => Ok(false),
        }
    }
}

struct CathodRayTube {
    cycle: usize,
    sprite: Sprite,
    pixels: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl CathodRayTube {
    fn new(lines: Lines<BufReader<File>>) -> CathodRayTube {
        CathodRayTube {
            cycle: 0,
            pixels: [[b'.'; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            sprite: Sprite {
                register_x: 1,
                next_add: None,
                lines,
            },
        }
    }

    fn try_tick(&mut self) -> std::io::Result<bool> {
        let register_x = self.sprite.register_x % DISPLAY_WIDTH as i32;
        let (x, y) = (
            self.cycle % DISPLAY_WIDTH,
            self.cycle / DISPLAY_WIDTH % DISPLAY_HEIGHT,
        );

        let line_read = self.sprite.try_tick()?;
        if line_read {
            if (register_x - x as i32).abs() <= 1 {
                self.pixels[y][x] = b'#';
            } else {
                self.pixels[y][x] = b'.';
            }
        }

        self.cycle += 1;

        Ok(line_read)
    }
}

impl std::fmt::Display for CathodRayTube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.pixels {
            writeln!(f, "{}", String::from_utf8_lossy(row))?;
        }
        Ok(())
    }
}

fn part2(reader: BufReader<File>) -> std::io::Result<()> {
    let mut crt = CathodRayTube::new(reader.lines());
    while crt.try_tick()? {}
    draw_display(&crt.pixels)?;
    Ok(())
}

fn draw_display(display: &[[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) -> std::io::Result<()> {
    let mut out = std::io::stdout();
    for row in display {
        out.write_all(row)?;
        out.write_all(b"\n")?;
    }
    out.flush()?;
    Ok(())
}
