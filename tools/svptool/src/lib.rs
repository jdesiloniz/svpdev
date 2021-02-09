#[macro_use]
extern crate clap;
use clap::App;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub input_filename: String,
    pub output_prefix: String,
    pub reverse_endianness: bool,
    pub split_binary: bool,
    pub split_size_kb: u16,
}

impl Config {
    pub fn new_from_args() -> Result<Config, ()> {
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();

        match (matches.value_of("INPUT"), matches.value_of("OUTPUT")) {
            (Some(input), Some(output)) => {
                let reverse_endianness = matches.occurrences_of("reverse_endianness") > 0;
                let split_binary = matches.occurrences_of("split_binary") > 0;

                let split_size_kb =
                    if let Some(kbs) = matches.value_of("split_size_kb").map(|v| v.parse()) {
                        if let Ok(size) = kbs {
                            if size > 4096 {
                                4096
                            } else {
                                size
                            }
                        } else {
                            512
                        }
                    } else {
                        512
                    };

                Ok(Config {
                    input_filename: input.to_string(),
                    output_prefix: output.to_string(),
                    reverse_endianness,
                    split_binary,
                    split_size_kb,
                })
            }
            _ => match App::from_yaml(yaml).print_long_help() {
                _ => Err(()),
            },
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.input_filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let endianness_fixed_contents = if config.reverse_endianness {
        swap_endianness(contents)
    } else {
        contents
    };

    let size = config.split_size_kb as u32 * 1024;
    let splitted_data = if config.split_binary {
        split_data(endianness_fixed_contents, size)
    } else {
        vec![endianness_fixed_contents]
    };

    let mut index = 0;

    for set_of_data in splitted_data {
        let filename = format!("{}_{}.o", config.output_prefix.clone(), index);
        let mut output_file = File::create(filename)?;
        output_file.write_all(&set_of_data)?;

        index = index + 1;
    }

    /*let contents = fs::read_to_string(config.input_filename)?;
    let tokens = tokens::tokenize(contents.as_str())?;

    let (symbol_table, equ_table) = assembly::extract_tables(&tokens);
    let opcodes = assembly::generate_opcodes(
        &tokens,
        &symbol_table,
        &equ_table,
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
    print_table(&equ_table, "Constants table");*/

    Ok(())
}

pub fn swap_endianness(data: Vec<u8>) -> Vec<u8> {
    let process_data = data.iter().fold((Vec::<u8>::new(), 0, 0), |acc, byte| {
        let (mut result, previous_byte, index) = acc;

        if index % 2 == 1 {
            result.push(*byte);
            result.push(previous_byte);

            (result, *byte, index + 1)
        } else {
            if index == data.len() - 1 {
                // If we got an odd data vector, we should still push the last byte following a 00
                // (we're dealing with words):
                result.push(0);
                result.push(*byte);
            }
            (result, *byte, index + 1)
        }
    });

    let (result, _, _) = process_data;

    result
}

pub fn split_data(data: Vec<u8>, size_per_slice: u32) -> Vec<Vec<u8>> {
    // This is not beautiful, but it works and I don't want to fight the type system
    // while avoiding mutability and a bad for loop:
    let mut result = Vec::<Vec<u8>>::new();

    for chunk in data.chunks(size_per_slice as usize) {
        result.push(chunk.to_vec());
    }

    result
}

#[cfg(test)]
mod functional_tests {
    use super::*;

    #[test]
    fn check_endianness_swap() {
        assert_eq!(swap_endianness(Vec::<u8>::new()), Vec::<u8>::new());
        assert_eq!(swap_endianness(vec![1, 2]), vec![2, 1]);
        assert_eq!(
            swap_endianness(vec![1, 2, 3, 4, 5, 6]),
            vec![2, 1, 4, 3, 6, 5]
        );
        assert_eq!(swap_endianness(vec![1, 2, 3]), vec![2, 1, 0, 3]);
        assert_eq!(swap_endianness(vec![0; 100000]), vec![0; 100000]);
    }

    #[test]
    fn check_data_split() {
        assert_eq!(split_data(Vec::<u8>::new(), 1), Vec::<Vec<u8>>::new());
        assert_eq!(split_data(vec![1, 2], 1), vec![vec![1], vec![2]]);
        assert_eq!(split_data(vec![1, 2], 2), vec![vec![1, 2]]);
        assert_eq!(split_data(vec![1, 2], 3), vec![vec![1, 2]]);
        assert_eq!(
            split_data(vec![1, 2, 3, 4, 5], 3),
            vec![vec![1, 2, 3], vec![4, 5]]
        );
        assert_eq!(
            split_data(vec![1, 2, 3, 4, 5, 6], 3),
            vec![vec![1, 2, 3], vec![4, 5, 6]]
        );
        assert_eq!(
            split_data(vec![1, 2, 3, 4, 5, 6, 7], 3),
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]
        );
        assert_eq!(split_data(vec![0; 100000], 100000), vec![vec![0; 100000]]);
    }
}
