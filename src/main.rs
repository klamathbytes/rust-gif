use std::io;
use std::io::prelude::*;
use std::fs::File;

fn as_u16_le(a: &[u8]) -> u16 {
    ((a[0] as u16) <<  0) |
    ((a[1] as u16) <<  8)
}

fn main() -> io::Result<()> {
    let mut f = File::open("avocado.gif")?;
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer)?;

    let header = String::from_utf8_lossy(&buffer[0..6]);
    let w = as_u16_le(&buffer[6..8]);
    let h = as_u16_le(&buffer[8..10]);
    // assuming color table exists and that color res is 8bits/pixel (max)
    let color_table_size = 2u32.pow((buffer[10] as u32 & 0x7)+1);

    println!("Header: {}, w: {}, h: {}, table size: {}", header, w, h, color_table_size);
    const GCT: usize = 13;

    for i in 0..color_table_size as usize {
        println!("r: {}, g: {}, b: {}", buffer[GCT+i*3], buffer[GCT+i*3+1], buffer[GCT+i*3+2]);
    }

    /*
     * @TODO: 1. change frame rates in each Graphic Control Extension Block
     *           (one for each frame after image blocks)
     *        2. invert colors or change colors
     *        3. make this work with a microcontroller or raspberry pi (dials)
     */

    Ok(())
}

