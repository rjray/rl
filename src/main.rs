fn main() {
    if let Err(err) = rl::parse_args().and_then(rl::run) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
