use super::errors;
use crate::asm::labels;
use crate::asm::macros;
use crate::asm::mnemonics;
use crate::asm::operators;
use std::error::Error;

#[derive(Debug, Copy, Clone)]
pub enum Token<'a> {
    Macro(macros::SspMacro),
    Mnemonic(mnemonics::SspMnemonic),
    Operator(operators::SspOperator<'a>),
    Label(&'a str),
    Invalid(&'a str),
}

impl<'a> Token<'a> {
    pub fn invalid_token(&self) -> Option<&'a str> {
        match self {
            Token::Invalid(s) => Some(s),
            _ => None,
        }
    }
}

pub fn tokenize(tokens: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let (tokens, errors) = split_raw_tokens(tokens).iter().fold(
        (Vec::new(), Vec::<Token>::new()),
        |(mut tokens, mut errors), token| {
            match token {
                _ if mnemonics::SspMnemonic::new(token).is_some() => {
                    tokens.push(Token::Mnemonic(mnemonics::SspMnemonic::new(token).unwrap()))
                }
                _ if macros::SspMacro::new(token).is_some() => {
                    tokens.push(Token::Macro(macros::SspMacro::new(token).unwrap()))
                }
                _ if operators::SspOperator::new(token).is_some() => {
                    tokens.push(Token::Operator(operators::SspOperator::new(token).unwrap()))
                }
                _ if labels::is_label(token) => {
                    tokens.push(Token::Label(token.trim_end_matches(":")))
                }
                _ => errors.push(Token::Invalid(token)),
            }

            (tokens, errors)
        },
    );

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(Box::new(errors::TokenizationError(format!(
            "Invalid tokens found: {:?}",
            errors
                .iter()
                .map(|e| Token::invalid_token(e).unwrap())
                .collect::<Vec<&str>>()
        ))))
    }
}

fn split_raw_tokens(raw_contents: &str) -> Vec<&str> {
    raw_contents
        .lines()
        .map(|line| remove_comments(split_line(line)))
        .flatten()
        .collect()
}

pub fn split_line(line: &str) -> Vec<&str> {
    line.split_whitespace()
        .map(|slice| slice.trim_matches(','))
        .fold(Vec::new(), |mut acc, mut x| {
            if x.contains(',') {
                let mut split = x.split(",").collect::<Vec<&str>>();
                acc.append(&mut split)
            } else {
                acc.push(&mut x)
            }

            acc
        })
}

pub fn remove_comments(tokens: Vec<&str>) -> Vec<&str> {
    let mut reached_comment: bool = false;

    tokens.iter().fold(Vec::new(), |mut acc, x| {
        reached_comment = reached_comment || x.starts_with("#");
        if !reached_comment {
            acc.push(x);
        }
        acc
    })
}
