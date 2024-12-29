use nom::types::CompleteStr;
use std;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::path::Path;
use vm::VM;

use crate::assembler::program_parsers::program;
use crate::vm;

// REPL: read evaluate print loop
pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM, // the vm the REPL will use to execute code
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![], // the buffer to store the commands, user can press up-arrow and see what they ran
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to the REPL!");
        loop {
            // allocate a string to store user input
            // TODO: figure out how to create this outside of the loop and re-use it every iteration
            let mut buffer = String::new();

            // blocking call until the user types in a command
            let stdin = io::stdin();

            // 'print!' doesn't automatically flush stdout like 'println!' does
            // so we do it manually here
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read from stdin");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Goodbye!");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        print!("{}, ", instruction);
                    }
                    println!("End of program listing");
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:?}", self.vm.registers);
                    println!("End of register listing");
                }
                ".hex" => {
                    self.vm.parse_hex_flag = !self.vm.parse_hex_flag;
                    println!(
                        "Hex parsing is now turned {}",
                        if self.vm.parse_hex_flag { "on" } else { "off" }
                    );
                }
                ".load_file" => {
                    print!("Please enter the path to the file you wish to load: ");
                    io::stdout().flush().expect("Unable to flush stdout");

                    let mut tmp = String::new();
                    stdin
                        .read_line(&mut tmp)
                        .expect("Unable to read from stdin");
                    let tmp = tmp.trim();
                    let filename = Path::new(&tmp);
                    let mut f = File::open(Path::new(&filename)).expect("File not found");
                    let mut contents = String::new();
                    f.read_to_string(&mut contents)
                        .expect("Something went wrong reading the file");
                    let program = match program(CompleteStr(&contents)) {
                        Ok((_, program)) => program,
                        Err(e) => {
                            println!("Error parsing program: {:?}", e);
                            continue;
                        }
                    };
                    self.vm.program.append(&mut program.to_bytes());
                }
                _ => {
                    if self.vm.parse_hex_flag {
                        let res = self.parse_hex(buffer);
                        match res {
                            Ok(bytes) => {
                                for byte in bytes {
                                    self.vm.add_byte(byte);
                                }
                            }
                            Err(_e) => {
                                println!("Invalid input. Please enter a valid hex string (4 groups of 2 hex characters)");
                            }
                        };
                    } else {
                        let program = match program(buffer.into()) {
                            Ok((_, program)) => program,
                            Err(_) => {
                                println!("Invalid input. Please enter a valid program");
                                continue;
                            }
                        };
                        self.vm.program.append(&mut program.to_bytes());
                    }
                    self.vm.run_once();
                }
            }
        }
    }

    // allows users to input hex strings to add to the VM's program
    // Accepts a hexadecimal string WITHOUT a leading `0x` and returns a Vec of u8
    // Example for a LOAD command: 00 01 03 E8
    fn parse_hex(&mut self, hex: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = hex.split(" ").collect::<Vec<&str>>();
        let mut res: Vec<u8> = vec![];
        for s in split {
            let byte = u8::from_str_radix(s, 16);
            match byte {
                Ok(b) => res.push(b),
                Err(e) => return Err(e),
            }
        }
        Ok(res)
    }
}
