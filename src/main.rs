mod meta;
mod statement;
use meta::do_meta_command;
use statement::{prepare_statement, PrepareResult};
use std::io;
fn main() -> io::Result<()> {
    loop {
        print_prompt();
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd)?;
        if cmd.trim().is_empty() {
            continue;
        }

        if cmd.starts_with('.') {
            match do_meta_command(cmd.trim()) {
                meta::MetaCommandResult::Exit => {
                    println!("exit, bye");
                    break;
                }
                meta::MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized meta command '{}'", cmd.trim());
                    continue;
                }
            };
        }

        match prepare_statement(cmd.trim()) {
            PrepareResult::Success => {
                println!("prepare success");
            }
            PrepareResult::UnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'", cmd.trim());
            }
        };
    }

    Ok(())
}

fn print_prompt() {
    print!("db > ")
}
