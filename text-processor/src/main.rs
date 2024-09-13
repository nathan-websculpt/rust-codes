
use pdf::document::PdfDocument;
use pdf::page::PdfPage;
extern crate pdf;

fn main() {
    println!("Enter the path to the PDF file:");
    let mut file_path = String::new();
    std::io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");
    let file_path = file_path.trim();

    if std::path::Path::new(file_path).exists() {
        process_pdf(file_path);
    } else {
        println!("File not found");
    }
}

fn process_pdf(file_path: &str) {
    let file = std::fs::File::open(file_path).expect("Failed to open file");
    let doc = PdfDocument::from_file(file).expect("Failed to parse PDF");

    for page in doc.pages() {
        let page = page.expect("Failed to get page");
        // Process each page of the PDF here
        println!("Page: {}", page.number());

        let page_text = page.extract_text().expect("Failed to extract text");
        println!("\nPage text:\n{}", page_text);
    }
}
