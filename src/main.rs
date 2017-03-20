mod selecta;

use std::io;
use std::io::BufRead;

fn main() {
    let choices = read_choices();
    let choices = choices.iter().map(|x| &x[..]).collect::<Vec<&str>>();

    let matches = selecta::compute_match(&choices, &"foo");
    println!("{}", matches.get(0).unwrap_or(&""));
}

//fn main_loop(choices: &[&str]) -> String {
//}

fn read_choices() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = Vec::new();

    let mut stdin = stdin.lock();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        trim_eol(&mut input);
        if input.is_empty() { break; }
        lines.push(input);
    }

    lines
}

fn trim_eol(string: &mut String) {
    while let Some(x) = string.pop() {
        if x != '\n' && x != '\r' {
            string.push(x);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{trim_eol};

    #[test]
    fn trim_eol_test() {
        let mut x = "asdf\n".to_string();
        trim_eol(&mut x);
        assert_eq!("asdf", x);
    }
}
