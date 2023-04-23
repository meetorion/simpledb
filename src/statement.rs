pub enum Statement {
    Insert,
    Select,
}
pub enum PrepareResult {
    Success,
    UnrecognizedStatement,
}

pub fn prepare_statement(cmd: &str) -> PrepareResult {
    if cmd.starts_with("insert") {
        return PrepareResult::Success;
    }
    if cmd.starts_with("select") {
        return PrepareResult::Success;
    }
    PrepareResult::UnrecognizedStatement
}
