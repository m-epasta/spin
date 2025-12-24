use std::env::args;

fn main() {
    use std::process::ExitCode;

    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        eprintln!("Usage: rslox <script>");
        return ExitCode::from(64);
    } else if args.len() == 1 {
        runFile(&args[0]);
    } else {
        runPrompt();
    }
}

fn runPrompt(path: &str) -> Result<(), std::io::Error> {
    let mut input = String::new();
    loop {
        print!("> ");
        std::io::stdin().read_line(&mut input);
        if std::io::stdin().read_line(&mut input).is_err() { std::io::Error::from(std::io::ErrorKind::UnexpectedEof) };
        run(&input);
    }
}



fn runFile(args: &str) -> _ {
    todo!()
}

fn run(input: &str) -> _ {
    todo!()
}