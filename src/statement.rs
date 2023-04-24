pub enum StatementType {
    Insert,
    Select,
    Unrecognized,
}
pub enum PrepareResult {
    Success,
    UnrecognizedStatement,
}

pub struct Statement {
    statement_type: StatementType,
}

impl Default for Statement {
    fn default() -> Self {
        Statement {
            statement_type: StatementType::Unrecognized,
        }
    }
}

pub fn prepare_statement(cmd: &str, stmt: &mut Statement) -> PrepareResult {
    if cmd.starts_with("insert") {
        stmt.statement_type = StatementType::Insert;
        // println!("This is where we would do an insert.");
        return PrepareResult::Success;
    }
    if cmd.starts_with("select") {
        stmt.statement_type = StatementType::Select;
        // println!("This is where we would do a select.");
        return PrepareResult::Success;
    }
    PrepareResult::UnrecognizedStatement
}

pub fn execute_statement(stmt: Statement) {
    match stmt.statement_type {
        StatementType::Insert => println!("This is where we would do an insert."),
        StatementType::Select => println!("This is where we would do a select."),
        StatementType::Unrecognized => println!("Unrecognized statement type."),
    }
}
