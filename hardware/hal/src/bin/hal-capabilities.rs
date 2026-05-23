use hal::{AppleSiliconProvider, CapabilityProvider};
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_usage();
        return;
    }

    let pretty = !args.iter().any(|arg| arg == "--compact");
    let provider = AppleSiliconProvider::new();
    let snapshot = provider.snapshot();
    let payload = snapshot.to_control_plane_register();
    let output = if pretty {
        serde_json::to_string_pretty(&payload)
    } else {
        serde_json::to_string(&payload)
    };

    match output {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("failed to serialize capability payload: {error}");
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!(
        "Usage: hal-capabilities [--compact]\n\n\
Outputs a capability snapshot payload for the control plane registry.\n\n\
Options:\n\
  --compact  Emit compact JSON (default is pretty-printed).\n\
  -h, --help Show this help text."
    );
}
