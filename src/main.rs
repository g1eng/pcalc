use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    capacity: usize,
}

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
