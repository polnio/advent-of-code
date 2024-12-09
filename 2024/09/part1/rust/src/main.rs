#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum Token {
    #[default]
    Empty,
    Block(usize),
}

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        println!("Usage: cargo run -- <path>");
        std::process::exit(1);
    };

    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            println!("Error reading file: {err}");
            std::process::exit(1);
        }
    };

    let mut blocks = content
        .bytes()
        .enumerate()
        .filter_map(|(i, b)| match b {
            b'0'..=b'9' => Some((i, b - b'0')),
            _ => None,
        })
        .fold(Vec::new(), |mut acc, (i, b)| {
            for _ in 0..b {
                if i & 1 == 0 {
                    acc.push(Token::Block(i / 2));
                } else {
                    acc.push(Token::Empty);
                }
            }
            acc
        });

    let mut i = 0;
    while i + 1 < blocks.len() {
        match &blocks[i] {
            Token::Empty => {
                let end = loop {
                    match blocks.pop() {
                        Some(Token::Empty) => continue,
                        Some(Token::Block(b)) => break Some(b),
                        None => break None,
                    }
                };
                let Some(end) = end else {
                    break;
                };
                blocks[i] = Token::Block(end);
            }
            Token::Block(_) => {}
        };
        i += 1;
    }

    let result = blocks
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, t)| match t {
            Token::Empty => acc,
            Token::Block(b) => acc + i * b,
        });

    println!("{result}");
}
