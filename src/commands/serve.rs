pub fn run(matches: &clap::ArgMatches<'_>) {
    println!("serving");
    println!("from (serve) commands::server::run...");

    let iterator = matches.values_of("arguments").unwrap();

    let vec: Vec<&str> = iterator.collect();

    crate::process::start(vec.clone());
}
