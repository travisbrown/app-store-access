use swc_common::BytePos;
use swc_ecma_ast::{EsVersion, Expr};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax, lexer::Lexer};

pub mod json;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JavaScript parsing error")]
    EcmaParse(swc_ecma_parser::error::Error),
    #[error("JSON conversion error")]
    JsonConversion(#[from] json::Error),
    #[error("Invalid script length")]
    InvalidScriptLength(usize),
}

pub fn parse_js(script: &str, version: EsVersion) -> Result<Box<Expr>, Error> {
    let end_pos =
        BytePos(u32::try_from(script.len()).map_err(|_| Error::InvalidScriptLength(script.len()))?);

    let lexer = Lexer::new(
        Syntax::Es(EsSyntax::default()),
        version,
        StringInput::new(script, BytePos(0), end_pos),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    parser.parse_expr().map_err(Error::EcmaParse)
}
