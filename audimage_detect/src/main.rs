use image::*;
use std::io::{stdin, Stdin};
use std::{collections::BTreeMap, io::BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pixel_map: BTreeMap<u8, Vec<(u32, u32)>> = BTreeMap::new();

    let stdin = stdin();

    let (img_dimensions, pixel_map) = parse_stdin(stdin, &mut pixel_map);

    create_image(img_dimensions, pixel_map.clone());

    Ok(())
}

// this function parses the stdout lines into a luma_val:Vec<coord> pair BTree
fn parse_stdin(
    stdin: Stdin,
    pixel_map: &mut BTreeMap<u8, Vec<(u32, u32)>>,
) -> ((u32, u32), &mut BTreeMap<u8, Vec<(u32, u32)>>) {
    let mut img_dimensions: (u32, u32) = (0, 0);
    let mut curr_luma_val: u8 = 255;

    for line in stdin.lock().lines() {
        match line {
            Ok(output) => {
                println!("{}", output);
                if output.contains(":") {
                    // lines that begin with ":" are luma values
                    // e.g. :155
                    let luma_val = output.split(":").collect::<Vec<&str>>()[1]
                        .parse::<u8>()
                        .unwrap_or(255);
                    curr_luma_val = luma_val;
                } else if output.contains("x") {
                    // the only line that contains "x" is the first line which contains dimensions
                    // e.g. 250x250
                    let dimensions: Vec<&str> = output.split("x").collect();
                    let x: u32 = dimensions[0].parse().unwrap_or(0);
                    let y: u32 = dimensions[1].parse().unwrap_or(0);

                    img_dimensions = (x, y);
                } else if output.contains(",") {
                    // all other lines will contain coordinates split by ",", assuming clean data
                    let coords: Vec<&str> = output.split(",").collect();
                    if coords.len() == 2 {
                        let x: u32 = coords[0].parse().unwrap_or(0);
                        let y: u32 = coords[1].parse().unwrap_or(0);

                        // insert luma_val:coordinate pairs into the BTree
                        pixel_map
                            .entry(curr_luma_val)
                            .and_modify(|coords| coords.push((x, y)))
                            .or_insert(vec![(x, y)]);
                    }
                }

                // exit condition
                if output.contains(";") {
                    return (img_dimensions, pixel_map);
                }
            }
            Err(e) => eprintln!("Error parsing line: {}", e),
        }
    }

    (img_dimensions, pixel_map)
}

fn create_image(dimensions: (u32, u32), pixel_map: BTreeMap<u8, Vec<(u32, u32)>>) {
    let mut image = GrayImage::new(dimensions.0, dimensions.1);

    for entry in pixel_map {
        let luma_val = entry.0;
        let coords: Vec<(u32, u32)> = entry.1;

        for coord in coords {
            image.put_pixel(coord.0, coord.1, Luma([luma_val]));
        }
    }

    image.save("./test.jpg").expect("Unable to save image.");
}
