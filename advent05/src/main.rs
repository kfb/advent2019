use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "advent03", about = "Wire Manhattan distances.")]
struct Opt {
    /// Input file containing a newline-separated list of wire descriptions
    #[structopt(name = "FILE")]
    file_name: String,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum OpcodeMode {
    Position,
    Immediate,
}

fn clean_input(input: &str) -> Vec<String> {
    input.split(",").map(|x| x.replace("\n", "")).collect()
}

fn parse_opcode(opcode: &str) -> Result<(&str, Vec<OpcodeMode>), &str> {
    let parsed_opcode;
    let mut modes = vec![];

    // If the opcode only has a single char, leave it unchanged
    if opcode.len() == 1 {
        parsed_opcode = opcode;
    } else {
        // Otherwise, we'll need to do some string surgery to get the opcode value
        // We're assuming here that opcodes will only be one char and the "0" is
        // padding--so first let's check what the first digit is
        if &opcode[(opcode.len() - 2)..opcode.len()] == "99" {
            parsed_opcode = "99";
        } else if &opcode[(opcode.len() - 2)..(opcode.len() - 1)] != "0" {
            return Err("Not an opcode");
        } else {
            parsed_opcode = &opcode[(opcode.len() - 1)..];
        }
    }

    // Determine how far the instruction pointer moves

    // Extract the number of parameters for an opcode and use that to determine whether
    // each param should be in immediate or position mode
    let num_params = match parsed_opcode {
        "1" => 3,
        "2" => 3,
        "3" => 1,
        "4" => 1,
        "5" => 2,
        "6" => 2,
        "7" => 3,
        "8" => 3,
        "99" => 0,
        _ => 0,
    };

    // Initialise the modes to position mode (the default)
    for _ in 0..num_params {
        modes.push(OpcodeMode::Position);
    }

    // If there were paramter mode chars we need to process them now
    if opcode.len() > 2 {
        // Extract everything except the last two digits
        let opcode_modes = &opcode[0..opcode.len() - 2];

        // Now fill with as many parameter modes as we have
        for (idx, mode) in opcode_modes.chars().enumerate() {
            modes[opcode_modes.len() - idx - 1] = match mode {
                '0' => OpcodeMode::Position,
                '1' => OpcodeMode::Immediate,
                _ => panic!("Unknown opcode mode '{}'", mode),
            }
        }
    }

    // Return the opcode and modes
    Ok((parsed_opcode, modes))
}

fn get_param(instructions: &Vec<String>, ip: usize, mode: OpcodeMode) -> i32 {
    match mode {
        OpcodeMode::Position => {
            let address = usize::from_str(&instructions[ip]).unwrap();
            i32::from_str(&instructions[address]).unwrap()
        },
        OpcodeMode::Immediate => {
            i32::from_str(&instructions[ip]).unwrap()
        },
    }
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let file = File::open(opt.file_name)?;
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_line(&mut input)?;

    // Split the input into an array of strings, removing any newlines if they're there
    let mut instructions = clean_input(&input);

    // The instruction pointer
    let mut ip = 0;

    // Loop over and process each instruction
    while ip < instructions.len() {
        // Get the opcode and parameter modes
        let (opcode, modes) = match parse_opcode(&instructions[ip]) {
            Ok(tuple) => tuple,
            Err(_) => break
        };

        match opcode {
            "1" => {
                // Add
                let lhs_val = get_param(&instructions, ip + 1, modes[0]);
                let rhs_val = get_param(&instructions, ip + 2, modes[1]);
                let out_ptr = u32::from_str(&instructions[ip + 3]).unwrap();

                println!("{}:\tADD   {}, {}\t-> {}", ip, lhs_val, rhs_val, out_ptr);

                instructions[out_ptr as usize] = (lhs_val + rhs_val).to_string();

                ip += 4;
            },
            "2" => {
                // Multiply
                let lhs_val = get_param(&instructions, ip + 1, modes[0]);
                let rhs_val = get_param(&instructions, ip + 2, modes[1]);
                let out_ptr = u32::from_str(&instructions[ip + 3]).unwrap();

                println!("{}:\tMUL   {}, {}\t-> {}", ip, lhs_val, rhs_val, out_ptr);

                instructions[out_ptr as usize] = (lhs_val * rhs_val).to_string();

                ip += 4;
            },
            "3" => {
                // Input
                let out_ptr = u32::from_str(&instructions[ip + 1]).unwrap();

                println!("{}:\tIN    {}", ip, out_ptr);

                // Read the user input
                let mut value = String::new();
                io::stdin().read_line(&mut value).unwrap();

                // Strip the newline
                let cleaned_value = &value[0..(value.len() - 1)];

                instructions[out_ptr as usize] = u32::from_str(cleaned_value).unwrap().to_string();

                ip += 2;
            },
            "4" => {
                // Output
                let out_ptr = get_param(&instructions, ip + 1, modes[0]);

                println!("{}:\tOUT   {}", ip, out_ptr);
                println!("{}", out_ptr);

                ip += 2;
            },
            "5" => {
                // Jump if true
                let lhs_val = get_param(&instructions, ip + 1, modes[0]);
                let rhs_val = get_param(&instructions, ip + 2, modes[1]);

                println!("{}:\tJMPT  {}, {}", ip, lhs_val, rhs_val);

                if lhs_val != 0 {
                    ip = rhs_val as usize;
                } else {
                    ip += 3;
                }
            },
            "6" => {
                // Jump if false
                let lhs_val = get_param(&instructions, ip + 1, modes[0]);
                let rhs_val = get_param(&instructions, ip + 2, modes[1]);

                println!("{}:\tJMPF  {}, {}", ip, lhs_val, rhs_val);

                if lhs_val == 0 {
                    ip = rhs_val as usize;
                } else {
                    ip += 3;
                }
            },
            "7" => {
                // Less than
                let lhs_val = get_param(&instructions, ip + 1, modes[0]);
                let rhs_val = get_param(&instructions, ip + 2, modes[1]);
                let out_ptr = u32::from_str(&instructions[ip + 3]).unwrap();

                println!("{}:\tLT    {}, {}\t-> {}", ip, lhs_val, rhs_val, out_ptr);

                if lhs_val < rhs_val {
                    instructions[out_ptr as usize] = "1".to_string();
                } else {
                    instructions[out_ptr as usize] = "0".to_string();
                }

                ip += 4;
            },
            "8" => {
                // Equals
                let lhs_val = get_param(&instructions, ip + 1, modes[0]);
                let rhs_val = get_param(&instructions, ip + 2, modes[1]);
                let out_ptr = u32::from_str(&instructions[ip + 3]).unwrap();

                println!("{}:\tEQ    {}, {}\t-> {}", ip, lhs_val, rhs_val, out_ptr);

                if lhs_val == rhs_val {
                    instructions[out_ptr as usize] = "1".to_string();
                } else {
                    instructions[out_ptr as usize] = "0".to_string();
                }

                ip += 4;
            },
            "99" => {
                // Halt
                println!("{}:\tHALT", ip);
                break;
            },
            _ => {
                println!("{}:\tERR", ip);
                break;
            }
        }
    }

    // println!("{:?}", instructions);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_param() {
        let test_data = clean_input("1002,4,3,4,33");

        assert_eq!(get_param(&test_data, 1, OpcodeMode::Position), 33);
        assert_eq!(get_param(&test_data, 1, OpcodeMode::Immediate), 4);
    }

    #[test]
    fn test_parse_opcode() {
        assert_eq!(parse_opcode("1"), ("1", vec![OpcodeMode::Position, OpcodeMode::Position, OpcodeMode::Position]));
        assert_eq!(parse_opcode("2"), ("2", vec![OpcodeMode::Position, OpcodeMode::Position, OpcodeMode::Position]));
        assert_eq!(parse_opcode("3"), ("3", vec![OpcodeMode::Position]));
        assert_eq!(parse_opcode("4"), ("4", vec![OpcodeMode::Position]));
    }
}
