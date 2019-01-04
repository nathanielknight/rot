const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn char_number(c: char) -> u8 {
    let target: char = c.to_ascii_lowercase();
    ALPHABET.find(target).unwrap() as u8
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
    n: i32,
}

impl Rotator {
    fn new(n: i32) -> Self {
        Rotator { n }
    }

    fn rot(&self, c: char) -> char {
        if !c.is_alphabetic() {
            return c;
        }
        let c_number = char_number(c);
        let new_c_number = (((c_number as i32 + self.n) % 26) + 26) % 26;
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

fn get_input() -> String {
    use std::io::Read;
    let argv: Vec<String> = std::env::args().into_iter().collect();
    match argv.len() {
        1 => {
            let mut result = String::new();
            let inp = std::io::stdin();
            let mut handle = inp.lock();
            handle.read_to_string(&mut result).unwrap();
            result
        }
        2 => {
            let fname = &argv[1];
            std::fs::read_to_string(fname).unwrap()
        }
        _ => {
            println!("usage: rot [filename]");
            std::process::exit(1);
        }
    }
}

fn main() {
    let input = get_input();
    let rotator = Rotator::new(13);

    let output: String = rotator.rot_str(&input);
    print!("{}", output);
}
