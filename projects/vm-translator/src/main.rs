use std::fs;

use vm_translator::{code_gen, parser};

// program overview
//
// == parser
// 1. load the .vm file
// 2. parse each line
//   a. ignore whitespace (comments indicated by "//")
//   b. validate & then parse line to target command
//   c. return Vector<Command> for the next step
//
// == code_gen
// 1. receive Vector<Command>
// 2. convert each command into ASM block
//   a. ASM block prefixed with VM command that generated it. format to be
//      `L{line_number}: {vm_command}`
//   b. write however many ASM lines needed
//   c. if there are more ASM blocks to come, append a newline to break up
//      blocks
//
// == main
// arguments:
//  - <file...> :: one or more files to convert into asm
//
// 1. will load `filename.vm`
// 2. will parse target file to commands using `parser`
// 3. will convert commands to ASM using `code_gen`
// 4. will save the result in `filename.asm`

fn handle_file(file_path: &str) -> Result<(), String> {
    let file = fs::read_to_string(file_path)
        .map_err(|x| format!("failed to read '{}' with error: {}", file_path, x))?;

    let jack_file = parser::JackFile::new(&file);
    let mut result = vec![];
    let mut errors = vec![];

    for (line_number, command) in jack_file.commands() {
        let hack_assembly = code_gen::generate(command);

        match hack_assembly {
            Ok(hack_assembly) => {
                result.push(format!("\nL{}: {}", line_number, command));

                for instruction in hack_assembly {
                    result.push(format!("{}", instruction));
                }
            }
            Err(err) => {
                errors.push(err);
            }
        }
    }

    if errors.len() == 0 {
        result.iter().for_each(|line| println!("{}", line));

        Ok(())
    } else {
        errors.iter().for_each(|e| println!("{}", e));

        Err(format!("parsing failed with {} errors", errors.len()))
    }
}

fn main() -> Result<(), String> {
    // TODO: parse command line args
    let file_names = vec![
        "/home/oliver/ghq/github.com/OliverNChalk/nand2tetris/projects/07/StackArithmetic/SimpleAdd/SimpleAdd.vm",
        "/home/oliver/ghq/github.com/OliverNChalk/nand2tetris/projects/07/StackArithmetic/StackTest/StackTest.vm",
        "/home/oliver/ghq/github.com/OliverNChalk/nand2tetris/projects/07/MemoryAccess/BasicTest/BasicTest.vm",
        "/home/oliver/ghq/github.com/OliverNChalk/nand2tetris/projects/07/MemoryAccess/StaticTest/StaticTest.vm",
        "/home/oliver/ghq/github.com/OliverNChalk/nand2tetris/projects/07/MemoryAccess/PointerTest/PointerTest.vm",
    ];

    // for each file, load
    let first_file = *file_names.get(0).unwrap();

    handle_file(first_file)
}
