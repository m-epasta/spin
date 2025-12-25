use std::{
    env,
    fs,
    io::{self, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: rslox [script]");
        ExitCode::from(64)
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("{}", e);
                ExitCode::from(65)
            }
        }
    } else {
        if let Err(e) = run_prompt() {
            eprintln!("{}", e);
            ExitCode::from(65)
        } else {
            ExitCode::SUCCESS
        }
    }
}


fn run_file(path: &str) -> io::Result<()> {
    let bytes = fs::read(path)?;
    let source = String::from_utf8_lossy(&bytes);
    run(&source);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?; // required in Rust

        line.clear();
        if stdin.read_line(&mut line)? == 0 {
            break; // EOF
        }

        run(&line);
    }

    Ok(())
}

fn run(source: &str) {
    // Placeholder for Scanner
    println!("{}", source);
}
