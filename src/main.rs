mod args;
mod notes;

//handle erreors
fn main() {
    let matches = args::parse_args();
    if let Err(e) = args::handle_matches(matches){
        eprintln!("An error occurred: {}", e);
    }
}