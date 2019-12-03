use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "advent02", about = "Process Intcode.")]
struct Opt {
    /// Input file containing a newline-separated list of module masses
    #[structopt(name = "FILE")]
    file_name: String,
}

fn tokenise(input: &str) -> Vec<u32> {
    let values: Vec<&str> = input.split(",").collect();
    let int_values: Vec<u32> = values.iter().map(|&value|
        u32::from_str(&value.replace("\n", "")).unwrap()
    ).collect();

    return int_values;
}

fn process(mut input: Vec<u32>) -> Vec<u32> {
    let mut idx = 0;

    loop {
        print!("{}:\t", idx);

        match input[idx] {
            1 => {
                let lhs_idx = input[idx + 1];
                let rhs_idx = input[idx + 2];
                let out_idx = input[idx + 3];

                let lhs = input[lhs_idx as usize];
                let rhs = input[rhs_idx as usize];

                println!("ADD {}, {}, {} # ({}, {})", lhs_idx, rhs_idx, out_idx, lhs, rhs);

                input[out_idx as usize] = lhs + rhs;

                idx += 4;
            },
            2 => {
                let lhs_idx = input[idx + 1];
                let rhs_idx = input[idx + 2];
                let out_idx = input[idx + 3];

                let lhs = input[lhs_idx as usize];
                let rhs = input[rhs_idx as usize];

                println!("MUL {}, {}, {} # ({}, {})", lhs_idx, rhs_idx, out_idx, lhs, rhs);

                input[out_idx as usize] = lhs * rhs;

                idx += 4;
            },
            99 => {
                println!("HALT");
                break;
            },
            _ => idx += 1,
        }
    }

    return input;
}

fn value_search(mut values: Vec<u32>, target: u32) -> Result<(u32, u32), ()> {
    for noun in 0..100 {
        for verb in 0..100 {
            values[1] = noun;
            values[2] = verb;

            let result = process(values.to_vec());

            if result[0] == target {
                return Ok((noun, verb));
            }
        }
    }

    Err(())
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let file = File::open(opt.file_name)?;
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    let values = tokenise(&first_line);
    let (noun, verb) = value_search(values, 19690720).unwrap();

    println!("noun = {}, verb = {}, 100 * {} + {} = {}", noun, verb, noun, verb, 100 * noun + verb);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process(tokenise("1,9,10,3,2,3,11,0,99,30,40,50")), vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(process(tokenise("1,0,0,0,99")), vec![2, 0, 0, 0, 99]);
        assert_eq!(process(tokenise("2,3,0,3,99")), vec![2, 3, 0, 6, 99]);
        assert_eq!(process(tokenise("2,4,4,5,99,0")), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(process(tokenise("1,1,1,4,99,5,6,0,99")), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}


