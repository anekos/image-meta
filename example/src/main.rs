use clap::Parser;
use regex::Captures;

use image_meta::load_from_file;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long, help = "Format: e.g) `%w x %h (%a)`")]
   format: Option<String>,

   files: Vec<String>,
}


fn main() {
    let args = Args::parse();

    let keys = regex::Regex::new("%(.)").unwrap();

    for file in args.files {
        let meta = load_from_file(&file).unwrap();
        if let Some(ref format) = args.format {
            let replaced = keys.replace_all(&format, |caps: &Captures| {
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
