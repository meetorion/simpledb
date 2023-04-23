pub enum MetaCommandResult {
    Exit,
    UnrecognizedCommand,
}

pub fn do_meta_command(buffer: &str) -> MetaCommandResult {
    // if buffer.chars().next() == "." {
    if buffer == ".exit" {
        MetaCommandResult::Exit
    } else {
        MetaCommandResult::UnrecognizedCommand
    }
}
