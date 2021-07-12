use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    capacity: usize,
}

fn check_prime(testant: u64, vec: &[u64]) -> bool {
    for q in vec {
        if testant % q == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let arg = Cli::from_args();
    let mut pr: Vec<u64> = vec![2];
    let mut i: u64 = 3;
    loop {
        if check_prime(i, &pr) {
            pr.push(i);
        }
        if pr.len() >= arg.capacity {
            break;
        }
        i += 2;
    }
    println!("{:?}", pr);
}
