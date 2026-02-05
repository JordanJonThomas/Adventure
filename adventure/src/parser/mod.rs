//! The parser module defines user sentence structure and parsing.

mod command;
mod tokens;
mod grammar;

use crate::errors::ParseError;
use tokens::tokenize;
use grammar::{parse_verb, parse_phrase};

pub use grammar::{NounPhrase, Verb, Preposition};
pub use tokens::Token;
pub use command::Command;

/// Attempts to convert user input from a string into a [`Command`]
pub fn parse(input: &str) -> Result<Command, ParseError> {
    let tokens = tokenize(input);
    if tokens.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let verb = parse_verb(&tokens[0].text)
        .ok_or(ParseError::UnknownVerb(tokens[0].text.to_string()))?;
    let remaining = &tokens[1..];

    parse_phrase(verb, remaining)
}

