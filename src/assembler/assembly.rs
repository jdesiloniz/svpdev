use super::errors;
use super::instructions;
use super::operations;
use crate::asm::macros;
use crate::asm::mnemonics;
use crate::asm::operators;
use crate::tokenization::tokens;
use std::collections::HashMap;
use std::error::Error;

const MAX_BINARY_SIZE: u32 = 0x20000;

pub fn extract_tables<'a>(
    tokens: &Vec<tokens::Token<'a>>,
) -> (HashMap<&'a str, u16>, HashMap<&'a str, u16>) {
    let mut current_address: u16 = 0;
    let mut current_org: bool = false;
    let mut current_dw: bool = false;
    let mut current_equ: bool = false;
    let mut current_equ_label: &str = "";

    let (symbols, equs, _) = tokens.iter().fold(
        (
            HashMap::<&'a str, u16>::new(),
            HashMap::<&'a str, u16>::new(),
            None::<tokens::Token>,
        ),
        |acc, token| {
            let (mut symbols, mut equs, prev_token) = acc;

            let size = match (token, prev_token) {
                (tokens::Token::Operator(_), Some(tokens::Token::Label(label))) => {
                    current_dw = false;
                    current_org = false;
                    symbols.insert(label, current_address);
                    0
                }
                (tokens::Token::Mnemonic(_), Some(tokens::Token::Label(label))) => {
                    current_dw = false;
                    current_org = false;
                    symbols.insert(label, current_address);
                    0
                }
                (tokens::Token::Invalid(_), Some(tokens::Token::Label(label))) => {
                    current_dw = false;
                    current_org = false;
                    symbols.insert(label, current_address);
                    0
                }

                (tokens::Token::Macro(macros::SspMacro::Org), _) => {
                    current_org = true;
                    current_dw = false;
                    current_equ = false;
                    0
                }
                (tokens::Token::Macro(macros::SspMacro::Dw), Some(tokens::Token::Label(label))) => {
                    current_org = false;
                    current_dw = true;
                    current_equ = false;
                    symbols.insert(label, current_address);
                    0
                }
                (tokens::Token::Macro(macros::SspMacro::Dw), _) => {
                    current_org = false;
                    current_dw = true;
                    current_equ = false;
                    0
                }
                (
                    tokens::Token::Macro(macros::SspMacro::Equ),
                    Some(tokens::Token::Label(label)),
                ) => {
                    current_org = false;
                    current_dw = false;
                    current_equ = true;
                    current_equ_label = label;
                    0
                }

                (tokens::Token::Operator(operators::SspOperator::Word(value)), _) => {
                    if current_org {
                        current_address = *value;
                        current_org = false;
                        0
                    } else if current_dw {
                        1
                    } else if current_equ {
                        equs.insert(current_equ_label, *value);
                        current_equ = false;
                        0
                    } else {
                        0
                    }
                }

                (
                    tokens::Token::Mnemonic(mnemonics::SspMnemonic::Sub(
                        mnemonics::SspMnemonicModifier::Immediate,
                    )),
                    _,
                ) => {
                    current_dw = false;
                    current_org = false;
                    2
                }
                (
                    tokens::Token::Mnemonic(mnemonics::SspMnemonic::Cmp(
                        mnemonics::SspMnemonicModifier::Immediate,
                    )),
                    _,
                ) => {
                    current_dw = false;
                    current_org = false;
                    2
                }
                (
                    tokens::Token::Mnemonic(mnemonics::SspMnemonic::Add(
                        mnemonics::SspMnemonicModifier::Immediate,
                    )),
                    _,
                ) => {
                    current_dw = false;
                    current_org = false;
                    2
                }
                (
                    tokens::Token::Mnemonic(mnemonics::SspMnemonic::And(
                        mnemonics::SspMnemonicModifier::Immediate,
                    )),
                    _,
                ) => {
                    current_dw = false;
                    current_org = false;
                    2
                }
                (
                    tokens::Token::Mnemonic(mnemonics::SspMnemonic::Or(
                        mnemonics::SspMnemonicModifier::Immediate,
                    )),
                    _,
                ) => {
                    current_dw = false;
                    current_org = false;
                    2
                }
                (
                    tokens::Token::Mnemonic(mnemonics::SspMnemonic::Ld(
                        mnemonics::SspMnemonicModifier::Immediate,
                    )),
                    _,
                ) => {
                    current_dw = false;
                    current_org = false;
                    2
                }
                (tokens::Token::Mnemonic(mnemonics::SspMnemonic::Bra), _) => {
                    current_dw = false;
                    current_org = false;
                    2
                }
                (tokens::Token::Mnemonic(mnemonics::SspMnemonic::Call), _) => {
                    current_dw = false;
                    current_org = false;
                    2
                }
                (tokens::Token::Mnemonic(_), _) => {
                    current_dw = false;
                    current_org = false;
                    1
                }

                (_, _) => {
                    current_dw = false;
                    current_org = false;
                    0
                }
            };

            current_address += size;

            (symbols, equs, Some(*token))
        },
    );
    (symbols, equs)
}

pub fn generate_opcodes<'a>(
    tokens: &Vec<tokens::Token<'a>>,
    symbols: &'a HashMap<&'a str, u16>,
    equs: &'a HashMap<&'a str, u16>,
    show_debug: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut binary: Vec<u8> = [0; MAX_BINARY_SIZE as usize].to_vec();

    let mut current_address: u32 = 0;
    let mut current_operation: Option<operations::Operation> = None;
    let mut errors: Vec<errors::AssemblyError> = Vec::new();
    let mut max_address: u32 = 0;

    for token in tokens.iter() {
        if max_address > MAX_BINARY_SIZE {
            return Err(Box::new(errors::AssemblyError(
                "Exceeded max binary size.".to_string(),
            )));
        }
        let mut proper_token: tokens::Token = *token;

        proper_token = match token {
            tokens::Token::Operator(operators::SspOperator::LabelRef(label))
                if symbols.contains_key(label) =>
            {
                symbols
                    .get(label)
                    .map(|addr| tokens::Token::Operator(operators::SspOperator::Word(*addr)))
                    .unwrap_or_else(|| proper_token)
            }

            tokens::Token::Operator(operators::SspOperator::LabelRef(label))
                if equs.contains_key(label) =>
            {
                equs.get(label)
                    .map(|value| tokens::Token::Operator(operators::SspOperator::Word(*value)))
                    .unwrap_or_else(|| proper_token)
            }

            _ => proper_token,
        };

        match (&current_operation, &proper_token) {
            // Ignoring labels from token list
            (_, tokens::Token::Label(_)) => (),

            // **** Macros ****

            // New macro
            (None, tokens::Token::Macro(m))
            | (Some(operations::Operation::Macro(macros::SspMacro::Dw)), tokens::Token::Macro(m)) => {
                current_operation = Some(operations::Operation::new_macro(m))
            }

            // Org macro
            (
                Some(operations::Operation::Macro(macros::SspMacro::Org)),
                tokens::Token::Operator(operators::SspOperator::Word(value)),
            ) => {
                current_operation = None;
                current_address = *value as u32 * 2;
            }

            // Equ macro
            (
                Some(operations::Operation::Macro(macros::SspMacro::Equ)),
                tokens::Token::Operator(operators::SspOperator::Word(_)),
            ) => {
                current_operation = None;
            }

            // Dw macro
            (
                Some(operations::Operation::Macro(macros::SspMacro::Dw)),
                tokens::Token::Operator(operators::SspOperator::Word(value)),
            ) => {
                let bytes = instructions::Instruction::word_to_bytes(*value);

                binary[current_address as usize] = bytes[0];
                binary[(current_address + 1) as usize] = bytes[1];
                current_address += 2;
            }

            // Dw byte will write a word either way (introduced for compatibility reasons)
            (
                Some(operations::Operation::Macro(macros::SspMacro::Dw)),
                tokens::Token::Operator(operators::SspOperator::Byte(value)),
            ) => {
                binary[current_address as usize] = 0;
                binary[(current_address + 1) as usize] = *value;
                current_address += 2;
            }

            // **** Instructions ****
            (None, tokens::Token::Mnemonic(m)) => {
                let instruction = instructions::Instruction::new(*m);
                if instruction.is_complete() {
                    if show_debug {
                        println!("Complete instruction: {:?}", instruction);
                    }

                    let opcodes = instruction.build()?;

                    for opcode in opcodes.iter() {
                        binary[current_address as usize] = *opcode;
                        current_address += 1;
                    }
                    current_operation = None;
                } else {
                    current_operation = Some(operations::Operation::new_instruction(&*m));
                }
            }

            (Some(operations::Operation::Instruction(i)), proper_token)
                if i.validate_op(proper_token) =>
            {
                let instruction = i.new_with_op(proper_token);

                if instruction.is_complete() {
                    if show_debug {
                        println!("Complete instruction: {:?}", instruction);
                    }

                    let opcodes = instruction.build()?;

                    for opcode in opcodes.iter() {
                        binary[current_address as usize] = *opcode;
                        current_address += 1;
                    }
                    current_operation = None;
                } else {
                    current_operation = Some(operations::Operation::Instruction(instruction));
                }
            }

            _ => errors.push(errors::AssemblyError(format!(
                "Invalid token: {:?} for instruction {:?}",
                proper_token, current_operation
            ))),
        }

        if max_address < current_address {
            max_address = current_address;
        }
    }

    binary.resize(max_address as usize, 0);

    if errors.is_empty() {
        Ok(binary)
    } else {
        Err(Box::new(errors::AssemblyError(format!(
            "{}",
            error_list(&errors)
        ))))
    }
}

pub fn error_list(errors: &Vec<errors::AssemblyError>) -> String {
    errors.iter().fold(String::new(), |mut acc, error| {
        acc.push_str(format!("{}", error).as_str());
        acc.push_str("\n");
        acc
    })
}
