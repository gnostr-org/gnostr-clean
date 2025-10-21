use gnostr_clean::run_cleaner;
use gnostr_clean::Options;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // parse in our options from the command line args
    let options = Options::from(env::args_os());

    // Run the cleaning process using the library function
    run_cleaner(options)
}