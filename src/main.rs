use std::env;
use midly::Smf;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    }
    
    let file:&str = &args[1];

    let smf = Smf::parse(include_bytes!(file)).expect("Failed to read midi file!");

    for (i, track) in smf.tracks.iter().enumerate() {
        println!("track {} has {} events", i, track.len());
    }
}
