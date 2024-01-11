use lopdf::Document;
use std::io::{Error, ErrorKind};

fn invert_pdf_colors(path: String) -> Result<(), std::io::Error> {
    let _doc = Document::load(path)
        .or_else(|e| {
            Err(Error::new(
                ErrorKind::Other,
                format!("input error, failed to load pdf: {}", e),
            ))
        })
        .unwrap();

    return Ok(());
}

fn main() {
    // could one-line this 5 line block
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprint!("please provide a pdf to invert");
        std::process::exit(1)
    }

    let _result = invert_pdf_colors(args[1].to_string());
}
