use std::env;
use std::fs::{self, File};
use std::io::BufWriter;
use jpeg_to_pdf::JpegToPdf;

fn main() {
    let (files, path) = get_arguments();
    create_pdf(files, path);
}

fn get_arguments() -> (Vec<String>, String) {
    let args: Vec<String> = env::args().collect();

    let mut files: Vec<String> = Vec::new();
    for i in 1..args.len()-1 {
        files.push(args[i].to_string());
    }

    let path = args[args.len()-1].clone();
    (files, path)
}

fn create_pdf(jpeg_names: Vec<String>, path: String) {
    let out_file = File::create(path+"/result.pdf")
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
