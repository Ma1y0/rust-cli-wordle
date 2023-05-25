use colored::Colorize;
use wordle::{check, get_random_world, validate, GuessResult, ColorOutput};

fn main() {
    let mut guesses: Vec<ColorOutput> = vec![]; 
    if let Some(word) = get_random_world() {
        println!("{}", word);
        while guesses.len() < 5 {
            println!("_____________________________________");
            for i in &guesses {
                println!("{}", i);
            }

            let mut buf = String::new();
            std::io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read input");
            buf = buf.to_lowercase();
            if !validate(&buf) {
                println!("Not a valid word");
                continue;
            }

            match check(&buf, &word) {
                GuessResult::Correct => {
                    println!("{}", "You won".to_string().bright_green());
                    std::process::exit(0);
                },
                GuessResult::Incorrect(a) => {
                    println!("{}", a);
                    guesses.push(a);
                }
            }
            println!("{}", "Your lost".to_string().bright_red());

        }
    } else {
        eprintln!(":(")
    }
}
