use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "advent01", about = "Calculate fuel masses for Santa's modules.")]
struct Opt {
    /// Don't account for the mass of the fuel
    #[structopt(short, long = "free-fuel")]
    free_fuel: bool,

    /// Input file containing a newline-separated list of module masses
    #[structopt(name = "FILE")]
    file_name: String,
}

fn fuel_for_mass(mass: i32, free_fuel: bool) -> i32 {
    let fuel_mass: i32 = (mass / 3) - 2;

    if free_fuel {
        return fuel_mass
    }

    if fuel_mass < 0 {
        return 0;
    }

    return fuel_mass + fuel_for_mass(fuel_mass, free_fuel);
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let file = File::open(opt.file_name)?;
    let reader = BufReader::new(file);

    let mut total_fuel = 0;
    let mut total_mass = 0;

    for line in reader.lines() {
        let mass: i32 = line.unwrap().parse().unwrap();

        total_mass += mass;
        total_fuel += fuel_for_mass(mass, opt.free_fuel);
    }

    println!("Fuel required for modules of combined mass {}: {} (free fuel: {})", total_mass, total_fuel, opt.free_fuel);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_for_mass() {
        assert_eq!(fuel_for_mass(12, true), 2);
        assert_eq!(fuel_for_mass(14, true), 2);
        assert_eq!(fuel_for_mass(1969, true), 654);
        assert_eq!(fuel_for_mass(100756, true), 33583);
    }

    #[test]
    fn test_fuel_for_mass_incl_fuel() {
        assert_eq!(fuel_for_mass(12, false), 2);
        assert_eq!(fuel_for_mass(1969, false), 966);
        assert_eq!(fuel_for_mass(100756, false), 50346);
    }
}
