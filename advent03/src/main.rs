use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use structopt::StructOpt;

use advent03::point::Point;
use advent03::wire::{Wire, WireSegment};

#[derive(Debug, StructOpt)]
#[structopt(name = "advent03", about = "Wire Manhattan distances.")]
struct Opt {
    /// Input file containing a newline-separated list of wire descriptions
    #[structopt(name = "FILE")]
    file_name: String,
}

fn offset_from_direction(direction: &str) -> Result<Point, &str> {
    let (dir, distance_str) = direction.split_at(1);
    let distance = f32::from_str(&distance_str).unwrap();

    match dir {
        "U" => Ok(Point::new(0.0, distance)),
        "D" => Ok(Point::new(0.0, -distance)),
        "L" => Ok(Point::new(-distance, 0.0)),
        "R" => Ok(Point::new(distance, 0.0)),
        _ => Err("Unexpected direction"),
    }
}

fn parse_directions(directions: &str) -> Wire {
    let mut wire = Wire::new();
    let mut current_pos = Point::new(0.0, 0.0);
    let offsets = directions.split(",").map(|dir| offset_from_direction(dir).unwrap());

    for offset in offsets {
        wire.push(WireSegment::new(current_pos.x, current_pos.y, offset.x, offset.y));
        current_pos = current_pos.add(&offset);
    }

    wire
}

fn count_steps_to_intersection(wire: &Wire, intersection: &Point) -> Result<f32, ()> {
    let mut steps = 0.0;

    for segment in wire.segments() {
        if segment.contains_point(intersection) {
            steps += segment.manhattan(&intersection);
            return Ok(steps);
        } else {
            steps += segment.len();
        }
    }

    Err(())
}

fn min_manhattan_distance(wires: &Vec<Wire>) -> f32 {
    let points = wires[0].intersection_points(&wires[1]);

    let origin = Point::new(0.0, 0.0);
    let mut min_distance = points[0].manhattan(&origin);

    for point in &points {
        let distance = point.manhattan(&origin);
        if distance < min_distance {
            min_distance = distance;
        }
    }

    min_distance
}

fn min_combined_steps(wires: &Vec<Wire>) -> f32 {
    let points = wires[0].intersection_points(&wires[1]);
    let mut min_steps = -1.0;

    for point in &points {
        let wire_1_steps = count_steps_to_intersection(&wires[0], &point).unwrap();
        let wire_2_steps = count_steps_to_intersection(&wires[1], &point).unwrap();
        let num_steps = wire_1_steps + wire_2_steps;

        if (min_steps == -1.0) || (num_steps < min_steps) {
            min_steps = num_steps;
        }
    }

    min_steps
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let file = File::open(opt.file_name)?;
    let reader = BufReader::new(file);

    let mut wires: Vec<Wire> = vec![];

    for line in reader.lines() {
        wires.push(parse_directions(&line.unwrap()));
    }

    println!("Min. Manhattan distance: {}", min_manhattan_distance(&wires));
    println!("Total min steps: {}", min_combined_steps(&wires));

    Ok(())
}
