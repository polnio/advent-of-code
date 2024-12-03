use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res, IResult};

fn parse_do(input: &str) -> IResult<&str, &str> {
    tag("do()")(input)
}

fn parse_dont(input: &str) -> IResult<&str, &str> {
    tag("don't()")(input)
}

fn parse_mul(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("mul(")(input)?;
    let (input, a) = map_res(digit1, str::parse::<u32>)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, b) = map_res(digit1, str::parse::<u32>)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, a * b))
}

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("Usage: cargo run -- <path>");
        std::process::exit(1);
    };

    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    };

    let mut input = content.as_str();
    let mut sum = 0;
    let mut reading = true;
    while input.len() > 0 {
        if let Ok((rest, _)) = parse_do(input) {
            input = rest;
            reading = true;
            continue;
        }
        if let Ok((rest, _)) = parse_dont(input) {
            input = rest;
            reading = false;
            continue;
        }
        if !reading {
            input = &input[1..];
            continue;
        }
        let Ok((rest, n)) = parse_mul(input) else {
            input = &input[1..];
            continue;
        };
        sum += n;
        input = rest;
    }

    println!("{}", sum);
}
