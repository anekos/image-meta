
use clap::{Arg, app_from_crate, crate_name, crate_version, crate_authors, crate_description};
use regex::Captures;

use image_meta::load_from_file;



fn main() {
    let args = app_from_crate!()
        .arg(Arg::with_name("format")
             .help("Database name")
             .short("f")
             .long("format")
             .takes_value(true))
        .arg(Arg::with_name("path")
             .min_values(0));

    let matches = args.get_matches();
    let format = matches.value_of("format");

    let keys = regex::Regex::new("%(.)").unwrap();
    let targets: Vec<&str> = matches.values_of("path").unwrap().collect();

    for target in targets {
        let meta = load_from_file(&target).unwrap();
        if let Some(format) = format {
            let replaced = keys.replace_all(format, |caps: &Captures| {
                match &caps[1] {
                    "w" => format!("{}", meta.dimensions.width),
                    "h" => format!("{}", meta.dimensions.height),
                    "a" => format!("{}", meta.animation_frames.unwrap_or(0)),
                    x => panic!("Unknown format character: {:?}", x),
                }

            });
            println!("{}", replaced);
        } else {
            println!("{:?}", meta);
        }
    }
}
