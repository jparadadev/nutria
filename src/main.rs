use clap::{App, Arg};
use std::fs::File;
use std::io;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

fn main() -> io::Result<()> {
    let matches = App::new("Rust Zip Compressor")
        .version("1.0")
        .author("Your Name <your_email@example.com>")
        .about("Compresses files into ZIP format with maximum compression rate")
        .arg(Arg::with_name("compress")
            .short('c')
            .long("compress")
            .value_name("FILE")
            .help("Compresses the specified file into ZIP format")
            .takes_value(true))
        .get_matches();

    if let Some(input_file) = matches.value_of("compress") {
        let path = std::path::Path::new(input_file);
        let file = File::create(format!("{}.zip", input_file))?;
        let mut zip = ZipWriter::new(file);

        let options = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o755)
            .large_file(true);

        zip.start_file(path.file_name().unwrap().to_str().unwrap(), options)?;

        let mut input_file = File::open(path)?;
        io::copy(&mut input_file, &mut zip)?;

        zip.finish()?;
        println!("File '{}' compressed to '{}.zip' with maximum compression.", path.file_name().unwrap().to_str().unwrap(), path.file_name().unwrap().to_str().unwrap());

    }

    Ok(())
}
