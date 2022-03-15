//! `package_file_verify_cli`
//! CLI for file verification

use clap::Parser;
use package_file_verify::verifiers::resolver::get_verifier;


#[derive(Parser)]
#[clap(about, version)]
struct Options {
    /// File path
    #[clap(short, long, value_name = "FULL_PATH")]
    file_path: String,
}

fn main() {
    env_logger::init();

    let options = Options::parse();

    let verifier = get_verifier()
        .expect("failed to get verifier method");
    
    verifier(&options.file_path);
}