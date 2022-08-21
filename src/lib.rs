mod back_end;

use std::error::Error;

use back_end::major::{self, Major, Class, Req};

pub fn run() -> Result<(), Box<dyn Error>> {
    let major = major::read_json("cs_major.json")?;
    println!("{:#?}", major);
    Ok(())
}
