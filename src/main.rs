const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn char_number(c: char) -> u8 {
    let target: char = c.to_ascii_lowercase();
    ALPHABET
        .find(target)
        .expect("arguments to char-number should be ascii-characters") as u8
}

#[test]
fn test_char_number() {
    for (i, c) in "abcdefghijklmnopqrstuvwxyz".chars().enumerate() {
        assert_eq!(char_number(c), i as u8);
    }
    for (i, c) in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate() {
        assert_eq!(char_number(c), i as u8);
    }
}

struct Rotator {
    shift: i32,
}

impl Rotator {
    fn new(shift: i32) -> Self {
        Rotator { shift }
    }

    fn rot(&self, c: char) -> char {
        if !c.is_ascii_alphabetic() {
            return c;
        }
        let c_number = char_number(c);
        let new_c_number = (((c_number as i32 + self.shift) % 26) + 26) % 26;
        let new_char = ALPHABET.chars().nth(new_c_number as usize).unwrap();
        if c.is_uppercase() {
            new_char.to_ascii_uppercase()
        } else {
            new_char
        }
    }

    fn rot_str(&self, inp: &str) -> String {
        inp.chars().map(|c| self.rot(c)).collect()
    }
}

#[test]
fn test_rot() {
    let msg = "Once there was a way; to get back home.";
    for n in -50..50 {
        let m = 26 - n;
        let fst = Rotator::new(n);
        let snd = Rotator::new(m);
        for c in msg.chars() {
            let rotd = snd.rot(fst.rot(c));
            assert_eq!(c, rotd);
        }
    }
}

struct Args {
    filepath: Option<String>,
    shift: i32,
}

fn parse_args() -> Result<Args, String> {
    let argv: Vec<String> = std::env::args().into_iter().collect();
    match argv.len() {
        1 => Ok(Args {
            filepath: None,
            shift: 13,
        }),
        2 => {
            let filepath = argv[1].to_owned();
            Ok(Args {
                filepath: Some(filepath),
                shift: 13,
            })
        }
        3 => {
            let flag = &argv[1];
            if flag != "--shift" && flag != "-s" {
                return Err("Invalid arguments".to_owned());
            }
            let shift_src = &argv[2];
            match shift_src.parse::<i32>() {
                Ok(shift) => Ok(Args {
                    shift,
                    filepath: None,
                }),
                Err(e) => Err(format!("--shift Expects a number: {}", e)),
            }
        }
        4 => {
            let flag = &argv[1];
            if flag != "--shift" && flag != "-s" {
                return Err("Invalid arguments".to_owned());
            }
            let shift_src = &argv[2];
            let filepath = argv[3].to_owned();
            match shift_src.parse::<i32>() {
                Ok(shift) => Ok(Args {
                    shift,
                    filepath: Some(filepath),
                }),
                Err(e) => Err(format!("--shift Expects a number: {}", e)),
            }
        }
        _ => Err("Invalid arguments".to_owned()),
    }
}

fn get_input(args: &Args) -> Outcome {
    use std::io::Read;
    match &args.filepath {
        None => {
            let mut result = String::new();
            let inp = std::io::stdin();
            let mut handle = inp.lock();
            handle
                .read_to_string(&mut result)
                .map_err(|e| format!("Error reading from stdin: {}", e))?;
            Ok(result)
        }
        Some(fname) => std::fs::read_to_string(&fname)
            .map_err(|e| format!("Error reading from '{}': {}", fname, e)),
    }
}

type Outcome = Result<String, String>;

fn action() -> Outcome {
    let args = parse_args()?;
    let input = match get_input(&args) {
        Ok(inp) => inp,
        Err(msg) => return Err(msg),
    };
    let shift = args.shift;
    let rotator = Rotator::new(shift);
    let output: String = rotator.rot_str(&input);
    Ok(output)
}

const USAGE: &'static str = "
usage:
  rot [filename] [-s|--shift n]
";

fn main() {
    match action() {
        Ok(outp) => print!("{}", outp),
        Err(msg) => {
            println!("{}", msg);
            print!("{}", USAGE);
            std::process::exit(1)
        }
    }
}
