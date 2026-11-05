//! statistics: the command line (ax_engine::cli: list, info, solve, batch, catalog, --version).

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(ax_engine::cli::run(statistics_ai999x::pack(), "statistics", &args));
}
