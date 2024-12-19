use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader, Lines},
    process::{ChildStdout, Command, Stdio},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // run the audimage binary
    // this is just for testing. eventually we will be handling stdout from minimodem --rx, but it
    // will be the same input
    let mut child_shell = Command::new("/home/jovie/projects/rust/audimage/target/debug/audimage")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Unable to run audimage.");

    let child_out = BufReader::new(child_shell.stdout.as_mut().unwrap());
    let input = child_out.lines();

    // reform the pixel_map BTree with luma_val:Vec<coord> pairs that is output by audimage
    let pixel_map = parse_input(input);
    println!("{:?}", pixel_map);

    Ok(())
}

// this function parses the stdout lines into a luma_val:Vec<coord> pair BTree
fn parse_input(
    lines: Lines<BufReader<&mut ChildStdout>>,
) -> ((u32, u32), BTreeMap<u8, Vec<(u32, u32)>>) {
    let mut result: BTreeMap<u8, Vec<(u32, u32)>> = BTreeMap::new();

    let mut curr_luma_val: u8 = 255;
    let mut img_dimensions: (u32, u32) = (0, 0);

    for line in lines {
        match line {
            Ok(output) => {
                if output.contains(":") {
                    // lines that begin with ":" are luma values
                    // e.g. :155
                    let luma_val = output.split(":").collect::<Vec<&str>>()[1]
                        .parse::<u8>()
                        .expect("Luma value is not a valid u8.");

                    curr_luma_val = luma_val;
                } else if output.contains("x") {
                    // the only line that contains "x" is the first line which contains dimensions
                    // e.g. 250x250
                    let dimensions: Vec<&str> = output.split("x").collect();
                    let x: u32 = dimensions[0].parse().expect("Unable to parse x dimension.");
                    let y: u32 = dimensions[1].parse().expect("Unable to parse y dimension.");

                    img_dimensions = (x, y);
                } else {
                    // all other lines will contain coordinates split by ",", assuming clean data
                    let coords: Vec<&str> = output.split(",").collect();
                    if coords.len() == 2 {
                        let x: u32 = coords[0].parse().expect("Unable to parse x coord.");
                        let y: u32 = coords[1].parse().expect("Unable to parse y coord.");

                        // insert luma_val:coordinate pairs into the BTree
                        result
                            .entry(curr_luma_val)
                            .and_modify(|coords| coords.push((x, y)))
                            .or_insert(vec![(x, y)]);
                    }
                }
            }
            Err(e) => eprintln!("Uh oh! Error processing line... {}", e),
        }
    }

    (img_dimensions, result)
}
