use std::env;

fn ackermann(m: u32, n: u32) -> u32 {
    match (m, n) {
        (0, n) => n + 1,                        
        (m, 0) => ackermann(m - 1, 1),         
        (m, n) => ackermann(m - 1, ackermann(m, n - 1)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    //if args.len() != 3 {
    //    eprintln!("Usage: {} <m> <n>", args[0]);
    //    std::process::exit(1);
    //}

    let m: u32 = args[1].parse().expect("m should be positive");
    let n: u32 = args[2].parse().expect("n should be positive");

    println!("Ackermann({}, {}) = {}", m, n, ackermann(m,n));
}
