#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(warnings)]

mod vm;
use vm::{
    VMInstance,
    regs::VMRegisters
};
mod compiler;

use crate::compiler::{CompileErrors, CompileErrorsList, compile, read_file};

fn main() {
    let mut instance = VMInstance::default();

    let error_handler = |e: (CompileErrors, CompileErrorsList)| -> !{
        println!("File compilation error! Info:");
        match e.0{
            CompileErrors::PARSE_FAILED => {
                println!("Failed to parse a number: {:?}", e.1.parse_int_error.unwrap());
                panic!()
            }
            CompileErrors::UNKNOWN_LABEL => {
                println!("Found a refrence to an undefined label with name: {}", e.1.unknown_label_name.unwrap());
                panic!()
            }
            CompileErrors::WRITE_TO_OUTPUT_FILE_FAILED => {
                println!("Failed to write to output file, error:\n{:?}", e.1.output_file_failure.unwrap());
                panic!()
            }
            CompileErrors::BAD_SYNTAX => {
                println!("Bad syntax found on word: {}", e.1.bad_syntax_opcode.unwrap());
                panic!()
            }
        }
    };

    let _ =  match compile(
        include_str!("test.ava").to_string(),
        "test.ab".to_string()
    ){
        Ok(r) => r,
        Err(e) =>{ 
            error_handler(e);
        }
    };

    let program = match read_file("test.ab"){
        Ok(r) => r,
        Err(e) => panic!("Failed to read file: {}", e)
    };

    instance.set_program(program);
    match instance.run(){
        Ok(()) => (),
        Err(e) => panic!("Virtual machine execution failure: {:?}", e)
    }

    println!("{:?}", instance.regs);
}