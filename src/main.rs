#![recursion_limit = "1024" ]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate clap;
extern crate ring;
extern crate data_encoding;

use clap::{App, Arg};
use data_encoding::hex;
use ring::rand::SecureRandom;


error_chain! {
    foreign_links {
        Ring(ring::error::Unspecified);
    }
    errors {}
}


fn run() -> Result<()> {
    let matches = App::new("randhex")
        .version(crate_version!())
        .about("Generates a secure set of n bytes and outputs a hex string")
        .arg(Arg::with_name("nbytes")
             .default_value("32")
             .takes_value(true)
             .help("number of random bytes to generate"))
        .get_matches();

    let n = matches.value_of("nbytes").unwrap().parse::<usize>().chain_err(|| "`nbytes` expects an integer")?;
    let rng = ring::rand::SystemRandom::new();
    let mut buf = vec![0u8; n];
    rng.fill(&mut buf)?;
    let s = hex::encode(&buf);
    println!("{} byte hex: {}", n, s);
    Ok(())
}


quick_main!(run);

