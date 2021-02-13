mod asm;
mod assembler;
mod tokenization;

#[macro_use]
extern crate clap;
use clap::App;

use assembler::assembly;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Write;
use tokenization::tokens;

pub struct Config {
    pub input_filename: String,
    pub output_filename: String,
    pub is_debug: bool,
    pub is_hex: bool,
    pub input_base_rom: Option<String>,
    pub should_fill: bool,
    pub max_binary_size_in_megs: u8,
}

impl Config {
    pub fn new_from_args() -> Result<Config, ()> {
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();

        match (matches.value_of("INPUT"), matches.value_of("OUTPUT")) {
            (Some(input), Some(output)) => {
                let is_debug = matches.occurrences_of("debug") > 0;
                let is_hex = matches.occurrences_of("hex") > 0;
                let should_fill = matches.occurrences_of("fill") > 0;
                let input_base_rom = matches.value_of("base").map(|b| b.to_string());

                let max_binary_size_in_megs = if matches.occurrences_of("1M") > 0 {
                    1
                } else if matches.occurrences_of("2M") > 0 {
                    2
                } else {
                    4
                };

                Ok(Config {
                    input_filename: input.to_string(),
                    output_filename: output.to_string(),
                    is_debug,
                    is_hex,
                    input_base_rom: input_base_rom,
                    should_fill: should_fill,
                    max_binary_size_in_megs: max_binary_size_in_megs,
                })
            }
            _ => match App::from_yaml(yaml).print_long_help() {
                _ => Err(()),
            },
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.input_filename)?;
    let tokens = tokens::tokenize(contents.as_str())?;

    let (symbol_table, equ_table, equb_table) = assembly::extract_tables(&tokens);
    let opcodes = assembly::generate_opcodes(
        &tokens,
        &symbol_table,
        &equ_table,
        &equb_table,
        config.is_debug,
        config.input_base_rom,
        config.should_fill,
        config.max_binary_size_in_megs,
    )?;

    let mut file = File::create(config.output_filename.clone())?;
    file.write_all(&opcodes)?;

    if config.is_hex {
        write_hex_file(format!("{}.{}", config.output_filename, "hex"), &opcodes)?;
    }

    if config.is_debug {
        print_debug_info(&tokens, &opcodes);
    }

    println!("");
    print_table(&symbol_table, "Symbol table");
    println!("");
    print_table(&equ_table, "Word constants table");
    print_table(&equb_table, "Byte constants table");

    Ok(())
}

pub fn write_hex_file(filename: String, opcodes: &Vec<u8>) -> Result<(), Box<dyn Error>> {
    let mut hex_file = File::create(filename)?;

    let (data, _, _) =
        opcodes
            .iter()
            .fold((String::new(), u16::min_value(), false), |acc, opcode| {
                let (result, value, is_lsb) = acc;
                if !is_lsb {
                    (result, ((*opcode as u16) << 8), true)
                } else {
                    let final_value = format!("{:04x}\n", (value | (*opcode as u16)));
                    (
                        format!("{}{}", result, final_value),
                        u16::min_value(),
                        false,
                    )
                }
            });

    hex_file.write_all(data.as_bytes())?;

    Ok(())
}

pub fn print_table<'a, T: fmt::UpperHex>(table: &HashMap<&'a str, T>, title: &str) {
    if table.len() > 0 {
        println!("**** {} ****", title);
        table
            .iter()
            .for_each(|(k, v)| println!("{} -> {:04X}", k, v))
    } else {
        println!("No elements found for {}", title);
    }
}

pub fn print_debug_info<'a>(tokens: &Vec<tokens::Token>, opcodes: &Vec<u8>) {
    if tokens.len() > 0 {
        println!("\nFound tokens:");
        for token in tokens {
            println!("{:?}", *token);
        }
    } else {
        println!("\nNo tokens found!");
    }

    println!("\nGenerated opcodes:");
    println!("{:02X?}", opcodes);
}
