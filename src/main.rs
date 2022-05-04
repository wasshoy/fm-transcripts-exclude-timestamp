mod lib;

pub use crate::lib::process_transcripts;

fn main() {
    process_transcripts::save_processed_scripts();
}
