use super::errors;
use super::instructions;
use super::operations;
use crate::asm::macros;
use crate::asm::mnemonics;
use crate::asm::operators;
use crate::asm::registers;
use crate::tokenization::tokens;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;

const MAX_BINARY_SIZE_1M: u64 = 0x100000;
const MAX_BINARY_SIZE_2M: u64 = 0x200000;
const MAX_BINARY_SIZE_4M: u64 = 0x400000;

pub fn extract_tables<'a>(
    tokens: &Vec<tokens::Token<'a>>,
) -> (HashMap<&'a str, u16>, HashMap<&'a str, u16>, HashMap<&'a str, u8>) {
    let mut current_address: u16 = 0;
    let mut current_org: bool = false;
    let mut current_dw: bool = false;
    let mut current_equ: bool = false;
    let mut current_equb: bool = false;
    let mut current_equ_label: &str = "";
    let mut current_mnemonic: Option<mnemonics::SspMnemonic> = None;

    let (symbols, equs, equbs, _) = tokens.iter().fold(
        (
            HashMap::<&'a str, u16>::new(),
            HashMap::<&'a str, u16>::new(),
            HashMap::<&'a str, u8>::new(),
            None::<tokens::Token>,
        ),
        |acc, token| {
            let (mut symbols, mut equs, mut equbs, prev_token) = acc;

            let size = match (token, prev_token) {
                (tokens::Token::Operator(_), Some(tokens::Token::Label(label))) => {
                    current_dw = false;
                    current_org = false;
                    symbols.insert(label, current_address);
                    0
                }
                (tokens::Token::Mnemonic(mnemonic), Some(tokens::Token::Label(label))) => {
                    current_dw = false;
                    current_org = false;
                    current_mnemonic = Some(*mnemonic);
                    symbols.insert(label, current_address);
                    1
                }
                (tokens::Token::Invalid(_), Some(tokens::Token::Label(label))) => {
                    current_dw = false;
                    current_org = false;
                    current_mnemonic = None;
                    symbols.insert(label, current_address);
                    0
                }

                (tokens::Token::Macro(macros::SspMacro::Org), _) => {
                    current_org = true;
                    current_dw = false;
                    current_equ = false;
                    current_equb = false;
                    current_mnemonic = None;
                    0
                }
                (tokens::Token::Macro(macros::SspMacro::Dw), Some(tokens::Token::Label(label))) => {
                    current_org = false;
                    current_dw = true;
                    current_equ = false;
                    current_equb = false;
                    current_mnemonic = None;
                    symbols.insert(label, current_address);
                    0
                }
                (tokens::Token::Macro(macros::SspMacro::Dw), _) => {
                    current_org = false;
                    current_dw = true;
                    current_equ = false;
                    current_equb = false;
                    current_mnemonic = None;
                    0
                }
                (
                    tokens::Token::Macro(macros::SspMacro::Equ),
                    Some(tokens::Token::Label(label)),
                ) => {
                    current_org = false;
                    current_dw = false;
                    current_equ = true;
                    current_equb = false;
                    current_mnemonic = None;
                    current_equ_label = label;
                    0
                }

                (
                    tokens::Token::Macro(macros::SspMacro::Equb),
                    Some(tokens::Token::Label(label)),
                ) => {
                    current_org = false;
                    current_dw = false;
                    current_equ = false;
                    current_equb = true;
                    current_mnemonic = None;
                    current_equ_label = label;
                    0
                }

                (
                    tokens::Token::Operator(operators::SspOperator::Word(_)),
                    Some(tokens::Token::Mnemonic(mnemonics::SspMnemonic::Ld(
                        mnemonics::SspMnemonicModifier::Reference,
                    ))),
                ) => {
                    // Special case for LD addr, a
                    current_mnemonic = None;
                    0
                }

                // OP A, addr needs to treat address words as size 1 instruction:
                (
                    tokens::Token::Operator(operators::SspOperator::Word(_)),
                    Some(tokens::Token::Operator(operators::SspOperator::Reg(
                        registers::SspGeneralRegister::A,
                    ))),
                ) if (current_mnemonic
                    == Some(mnemonics::SspMnemonic::Sub(
                        mnemonics::SspMnemonicModifier::Reference,
                    ))) =>
                {
                    // Special case for OP A, addr
                    current_mnemonic = None;
                    0
                }

                (
                    tokens::Token::Operator(operators::SspOperator::Word(_)),
                    Some(tokens::Token::Operator(operators::SspOperator::Reg(
                        registers::SspGeneralRegister::A,
                    ))),
                ) if (current_mnemonic
                    == Some(mnemonics::SspMnemonic::Cmp(
                        mnemonics::SspMnemonicModifier::Reference,
                    ))) =>
                {
                    // Special case for OP A, addr
                    current_mnemonic = None;
                    0
                }

                (
                    tokens::Token::Operator(operators::SspOperator::Word(_)),
                    Some(tokens::Token::Operator(operators::SspOperator::Reg(
                        registers::SspGeneralRegister::A,
                    ))),
                ) if (current_mnemonic
                    == Some(mnemonics::SspMnemonic::Add(
                        mnemonics::SspMnemonicModifier::Reference,
                    ))) =>
                {
                    // Special case for OP A, addr
                    current_mnemonic = None;
                    0
                }

                (
                    tokens::Token::Operator(operators::SspOperator::Word(_)),
                    Some(tokens::Token::Operator(operators::SspOperator::Reg(
                        registers::SspGeneralRegister::A,
                    ))),
                ) if (current_mnemonic
                    == Some(mnemonics::SspMnemonic::And(
                        mnemonics::SspMnemonicModifier::Reference,
                    ))) =>
                {
                    // Special case for OP A, addr
                    current_mnemonic = None;
                    0
                }

                (
                    tokens::Token::Operator(operators::SspOperator::Word(_)),
                    Some(tokens::Token::Operator(operators::SspOperator::Reg(
                        registers::SspGeneralRegister::A,
                    ))),
                ) if (current_mnemonic
                    == Some(mnemonics::SspMnemonic::Or(
                        mnemonics::SspMnemonicModifier::Reference,
                    ))) =>
                {
                    // Special case for OP A, addr
                    current_mnemonic = None;
                    0
                }

                (
                    tokens::Token::Operator(operators::SspOperator::Word(_)),
                    Some(tokens::Token::Operator(operators::SspOperator::Reg(
                        registers::SspGeneralRegister::A,
                    ))),
                ) if (current_mnemonic
                    == Some(mnemonics::SspMnemonic::Eor(
                        mnemonics::SspMnemonicModifier::Reference,
                    ))) =>
                {
                    // Special case for OP A, addr
                    current_mnemonic = None;
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
                        1
                    }
                }

                (tokens::Token::Operator(operators::SspOperator::Byte(value)), _) => {
                    // Dealing with EQUB
                    if current_equb {
                        equbs.insert(current_equ_label, *value);
                        current_equb = false;
                        0
                    } else {
                        1
                    }
                }

                (tokens::Token::Operator(operators::SspOperator::LabelRef(_)), _) => 1,

                (tokens::Token::Mnemonic(mnemonic), _) => {
                    current_dw = false;
                    current_org = false;
                    current_mnemonic = Some(*mnemonic);
                    1
                }

                (_, _) => {
                    current_dw = false;
                    current_org = false;
                    0
                }
            };

            current_address += size;

            (symbols, equs, equbs, Some(*token))
        },
    );
    (symbols, equs, equbs)
}

pub fn generate_opcodes<'a>(
    tokens: &Vec<tokens::Token<'a>>,
    symbols: &'a HashMap<&'a str, u16>,
    equs: &'a HashMap<&'a str, u16>,
    equbs: &'a HashMap<&'a str, u8>,
    show_debug: bool,
    input_base_rom: Option<String>,
    should_fill: bool,
    max_binary_size_in_megs: u8,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut binary: Vec<u8> = if let Some(ref base_file_path) = input_base_rom {
        let mut file = File::open(&base_file_path).expect("input base file not found");
        let metadata = fs::metadata(&base_file_path).expect("unable to read base file metadata");
        let file_size = metadata.len();

        if file_size > (max_binary_size_in_megs as u64) * 0x100000 {
            return Err(Box::new(errors::AssemblyError(
                "Exceeded max binary size.".to_string(),
            )));
        }

        let mut buffer = vec![0; metadata.len() as usize];
        file.read(&mut buffer).expect("buffer overflow");
        buffer.resize(MAX_BINARY_SIZE_4M as usize, 0);

        buffer
    } else {
        [0; MAX_BINARY_SIZE_4M as usize].to_vec()
    };

    let mut current_address: u64 = 0;
    let mut current_operation: Option<operations::Operation> = None;
    let mut errors: Vec<errors::AssemblyError> = Vec::new();
    let mut max_address: u64 = 0;

    for token in tokens.iter() {
        if max_address > max_binary_size_in_megs as u64 * 0x100000 {
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

            tokens::Token::Operator(operators::SspOperator::LabelRef(label))
                if equbs.contains_key(label) =>
            {
                equbs.get(label)
                    .map(|value| tokens::Token::Operator(operators::SspOperator::Byte(*value)))
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
                current_address = *value as u64 * 2;
            }

            // Equ macro
            (
                Some(operations::Operation::Macro(macros::SspMacro::Equ)),
                tokens::Token::Operator(operators::SspOperator::Word(_)),
            ) => {
                current_operation = None;
            }

            // Equb macro
            (
                Some(operations::Operation::Macro(macros::SspMacro::Equb)),
                tokens::Token::Operator(operators::SspOperator::Byte(_)),
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

    if !should_fill {
        if let Some(ref base_file_path) = input_base_rom {
            let metadata =
                fs::metadata(&base_file_path).expect("unable to read base file metadata");
            let file_size = metadata.len();

            binary.resize(file_size as usize, 0);
        } else {
            binary.resize(max_address as usize, 0);
        }
    } else {
        match max_binary_size_in_megs {
            1 => binary.resize(MAX_BINARY_SIZE_1M as usize, 0),
            2 => binary.resize(MAX_BINARY_SIZE_2M as usize, 0),
            _ => binary.resize(MAX_BINARY_SIZE_4M as usize, 0),
        }
    }

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
