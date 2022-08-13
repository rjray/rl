fn main() {
    if let Err(err) = rl::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
