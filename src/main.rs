//! pcalc - A simple primary calculator

use structopt::clap::{App, Arg};
use std::str::FromStr;
use std::process::exit;

fn check_prime(testant: u32, vec: &[u32]) -> bool {
    let mut i = 0;
    let limit_num = unsafe { f64::from(vec[vec.len() - 1]).sqrt().to_int_unchecked::<u32>() };
    while vec[i] <= limit_num {
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
        .version("0.3.2")
        .author("Suzume Nomura <suzume315[ATT]g00.g1e.org>")
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
            .help("popup last primary as a hex number")
            .required(false)
            )
        .arg(
            Arg::with_name("check")
            .short("c")
            .long("check")
            .help("check whether the number is a primary or not")
            .required(false)
            )
        .get_matches();

    let verbose = arg.is_present("verbose");
    let with_hex = arg.is_present("hex");
    let with_pop = arg.is_present("pop");
    let for_check = arg.is_present("check");
    let capacity = match arg.value_of("capacity"){
        Some(x) => usize::from_str(x).expect("invalid capacity number!"),
        None => {
            println!("invalid capacity number, abort!");
            return
        }
    };
    let num_limit = match arg.value_of("capacity"){
        Some(x) => u32::from_str(x).expect("invalid capacity number!"),
        None => {
            println!("invalid capacity number, abort!");
            return
        }
    };


    let mut pr: Vec<u32> = vec![2];
    let mut i: u32 = 3;
    if capacity > 0 && ! for_check {
        loop {
                if check_prime(i, &pr) {
                        pr.push(i);
                }
                if pr.len() >= capacity {
                        break;
                }
                i += 2;
        }
    } else if for_check {
        loop {
                if check_prime(i, &pr) {
                        pr.push(i);
                }
                if i >= num_limit {
                        break;
                }
                i += 2;
        }
    }

    let last = pr[pr.len() - 1];
    if verbose {
        println!("{:?}", pr);
    } else if with_pop || with_hex {
        if with_hex {
            println!("{:?}", format!("{:#x}", last));
        } else {
            println!("{}", last);
        }
    } 
    if for_check {
        if verbose {
            if last != num_limit {
                println!("{} is not a primary number", num_limit);
            } else {
                println!("{} is a primary number", num_limit);
            }
        }
        if last != num_limit {
            exit(1);
        }
    }
}
