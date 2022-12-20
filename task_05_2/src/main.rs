use std::ops::{Add, Div};

#[derive(Debug)]
pub struct WrappedU8(u8);

// write here something

fn main() -> Result<(), anyhow::Error> {
    println!("Start");

    // Step 1
    let x = WrappedU8(1) + WrappedU8(1);
    let x = x?;

    println!("1 + 1 = {}", x.0);

    // Step 2
    let x = WrappedU8(255) + WrappedU8(1);
    let x = x.unwrap_err();
    println!("255 + 1 = {}", x);

    // Step 3
    let x = WrappedU8(9) / WrappedU8(3);
    let x = x?;
    println!("9 / 3 = {}", x.0);

    // Step 4
    let x = WrappedU8(255) / WrappedU8(0);
    let x = x.unwrap_err();
    println!("255 / 0 = {}", x);

    println!("Finish");
    Ok(())
}
