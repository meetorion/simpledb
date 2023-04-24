mod meta;
mod statement;
use meta::do_meta_command;
use statement::{execute_statement, prepare_statement, PrepareResult};
use std::io::{self, Write};
fn main() -> io::Result<()> {
    loop {
        print_prompt();
        io::stdout().flush()?;
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
                    println!("Unrecognized command '{}'", cmd.trim());
                    continue;
                }
            };
        }

        let mut stmt: statement::Statement = statement::Statement::default();

        match prepare_statement(cmd.trim(), &mut stmt) {
            PrepareResult::Success => {
                // break;
            }
            PrepareResult::UnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'", cmd.trim());
                continue;
            }
        };

        execute_statement(stmt);
        println!("Executed.");
    }

    Ok(())
}

fn print_prompt() {
    print!("db > ")
}
