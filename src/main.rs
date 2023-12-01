use std::env;
use std::fs::{self, File};
use std::io::BufWriter;
use jpeg_to_pdf::JpegToPdf;

fn main() {
    let files = get_arguments();
    create_pdf(files);
}

fn get_arguments() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    let mut files: Vec<String> = Vec::new();
    for i in 1..args.len() {
        files.push(args[i].to_string());
    }
    files
}

fn create_pdf(jpeg_names: Vec<String>) {
    let out_file = File::create("result.pdf")
        .expect("Не могу инициализировать PDF");
    let mut files: Vec<Vec<u8>> = Vec::new();
    for item in jpeg_names {
        let file = fs::read(item)
            .expect("Не могу прочитать файл");
        files.push(file);
    }
    let iter: std::vec::IntoIter<Vec<u8>> = files.into_iter();
    JpegToPdf::new()
        .add_images(iter)
        .create_pdf(&mut BufWriter::new(out_file))
        .expect("Не могу создать PDF..");
}
