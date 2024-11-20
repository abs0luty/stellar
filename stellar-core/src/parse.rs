use lasso::Rodeo;

use crate::{ast::Statement, scan::{scan, ScanError}, token::{TokenStream, TokenStreamCursor}};

pub fn parse(stream: TokenStream) -> Result<Vec<Statement>, ParseError> {
    let mut cursor = stream.cursor();
    let mut statements = Vec::new();

    while !cursor.peek().is_eof() {
        statements.push(parse_statement(&mut cursor)?);
    }

    Ok(statements)
}

fn parse_statement(cursor: &mut TokenStreamCursor) -> Result<Statement, ParseError> {
    todo!()
}

enum ParseError {
}
