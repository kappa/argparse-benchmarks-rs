const HELP: &str = "\
USAGE: app [OPTIONS] --number NUMBER INPUT..

OPTIONS:
  --number NUMBER       Set a number (required)
  --opt-number NUMBER   Set an optional number
  --width WIDTH         Set a width (non-zero, default 10)

ARGS:
  <INPUT>               Input file
";

#[derive(Debug)]
struct AppArgs {
    number: u32,
    opt_number: Option<u32>,
    width: u32,
    input: Vec<std::path::PathBuf>,
}

fn parse_width(s: &str) -> Result<u32, String> {
    let w = s.parse().map_err(|_| "not a number")?;
    if w != 0 {
        Ok(w)
    } else {
        Err("width must be positive".to_string())
    }
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error: {}.", err);
            std::process::exit(1);
        }
    };
    if 10 < args.input.len() {
        println!("{:#?}", args.input.len());
    } else {
        println!("{:#?}", args);
    }
}

fn parse_args() -> Result<AppArgs, lexopt::Error> {
    use lexopt::prelude::*;

    let mut number = None;
    let mut opt_number = None;
    let mut width = 10;
    let mut input = Vec::new();

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                print!("{}", HELP);
                std::process::exit(0);
            }
            Long("number") => number = Some(parser.value()?.parse()?),
            Long("opt-number") => opt_number = Some(parser.value()?.parse()?),
            Long("width") => width = parser.value()?.parse_with(parse_width)?,
            Value(path) => input.push(path.into()),
            _ => return Err(arg.unexpected()),
        }
    }
    Ok(AppArgs {
        number: number.ok_or("missing required option --number")?,
        opt_number,
        width,
        input,
    })
}
