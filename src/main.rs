//! pcalc - A high-speed primary calculator
//!
//!


use structopt::clap::{App, Arg};
use std::str::FromStr;

fn check_prime(testant: u64, vec: &[u64]) -> bool {
    let mut i = 0;
    let limit = vec.len() - 1;
    while vec[i] < vec[limit] / 2 {
        if testant % vec[i] == 0 {
            return false;
        }
        i += 1;
    }
    if testant % vec[i] == 0 {
        return false;
    }
    return true;
}

fn main() {
    let arg = App::new("pcalc")
        .version("0.3.0")
        .author("Suzume Nomura")
        .arg(
            Arg::with_name("capacity")
            .value_name("LIMIT")
            .index(1)
            .help("calculation limit for primary number sequence")
            .required(true)
            )
        .arg(
            Arg::with_name("verbose")
            .long("verbose")
            .help("print all result for the calculation")
            .required(false)
            )
        .arg(
            Arg::with_name("get")
            .short("g")
            .long("get")
            .help("get number with a sequence")
            .required(false)
            )
        .arg(
            Arg::with_name("pop")
            .short("p")
            .long("pop")
            .help("popup last primary of the calculation")
            .required(false)
            )
        .arg(
            Arg::with_name("hex")
            .short("H")
            .help("print value as a hex number")
            .required(false)
            )
        .get_matches();

    let verbose = arg.is_present("verbose");
    let with_hex = arg.is_present("hex");
    let with_pop = arg.is_present("pop");
    let capacity = match arg.value_of("capacity"){
        Some(x) => usize::from_str(x).expect("invalid capacity number!"),
        None => {
            println!("invalid capacity number, abort!");
            return
        }
    };


    let mut pr: Vec<u64> = vec![2];
    let mut i: u64 = 3;
    loop {
        if check_prime(i, &pr) {
            pr.push(i);
        }
        if pr.len() >= capacity {
            break;
        }
        i += 2;
    }

    if verbose {
        println!("{:?}", pr);
    }
    if with_pop {
        let last = pr[pr.len() - 1];
        if with_hex {
            println!("{:?}", format!("{:#x}", last));
        } else {
            println!("{}", last);
        }
    }
}
