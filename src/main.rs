
extern crate clap;
use clap::App;

fn main() {
    App::new("fiction book library")
        .version("0.1.0")
        .about("Manages books")
        .author("seb")
        .get_matches();
}