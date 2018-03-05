use std::fs::File;
use std::process;
use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;

static FONT_FILENAME: &'static str = "./hershey-fonts/futural.jhf";
static MID: char = 'R';

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

pub fn run() -> Result<(), Box<Error>> {
    let f = File::open(FONT_FILENAME)?;

    for line_result in BufReader::new(f).lines() {
        let line = line_result?;
        let mut data = line.get(8..).unwrap().chars();
        println!("{:?} {:?} {:?} {:?}", line.get(0..5), line.get(5..8), line.get(8..), data);

        while let Some(cx) = data.next() {
            let cy = match data.next() {
              Some(c) => c,
              None => break,
            };

            let x = (MID as i8) - (cx as i8);
            let y = (MID as i8) - (cy as i8);

            println!("{:?},{:?}", x, y);
        }
    }

    Ok(())
}
