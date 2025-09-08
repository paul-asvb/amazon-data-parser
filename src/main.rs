use csv::{Reader, Writer};
use pdf_extract::extract_text;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;

fn extract_total_from_pdf(pdf_path: &Path) -> Result<String, Box<dyn Error>> {
    let text = extract_text(pdf_path)?;
    
    let total_regex = Regex::new(r"(?i)total[:\s]+([€$£]\s*)?(\d+[.,]\d{2})")?;
    
    if let Some(captures) = total_regex.find(&text) {
        return Ok(captures.as_str().to_string());
    }
    
    let amount_regex = Regex::new(r"([€$£]\s*)?(\d+[.,]\d{2})")?;
    let amounts: Vec<&str> = amount_regex.find_iter(&text).map(|m| m.as_str()).collect();
    
    if let Some(last_amount) = amounts.last() {
        return Ok(last_amount.to_string());
    }
    
    Ok("0.00".to_string())
}

fn process_invoices() -> Result<(), Box<dyn Error>> {
    let invoices_dir = "data/invoices";
    let output_file = "invoice_2021.csv";

    if !Path::new(invoices_dir).exists() {
        eprintln!("Directory {} does not exist", invoices_dir);
        return Ok(());
    }

    let mut output_writer = Writer::from_path(output_file)?;
    
    output_writer.write_record(&["filename", "total"])?;

    let entries = fs::read_dir(invoices_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("pdf") {
            println!("Processing invoice: {}", path.display());
            
            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            
            let total = extract_total_from_pdf(&path).unwrap_or_else(|_| "0.00".to_string());
            
            output_writer.write_record(&[filename, &total])?;
        }
    }

    output_writer.flush()?;
    println!("Successfully processed PDF invoices into {}", output_file);
    Ok(())
}

fn process_payments() -> Result<(), Box<dyn Error>> {
    let payments_dir = "data/payments";
    let output_file = "payments_2021.csv";

    if !Path::new(payments_dir).exists() {
        eprintln!("Directory {} does not exist", payments_dir);
        return Ok(());
    }

    let mut output_writer = Writer::from_path(output_file)?;
    let mut headers_written = false;

    let entries = fs::read_dir(payments_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            println!("Processing payment file: {}", path.display());
            
            let mut reader = Reader::from_reader(fs::File::open(&path)?);
            reader.headers()?; // Force reading headers
            
            if !headers_written {
                let headers = reader.headers()?;
                output_writer.write_record(headers)?;
                headers_written = true;
            }
            
            for result in reader.records() {
                let record = result?;
                output_writer.write_record(&record)?;
            }
        }
    }

    output_writer.flush()?;
    println!("Successfully merged payment files into {}", output_file);
    Ok(())
}

fn process_sales() -> Result<(), Box<dyn Error>> {
    let sales_dir = "data/sales";
    let output_file = "sales_2021.csv";

    if !Path::new(sales_dir).exists() {
        eprintln!("Directory {} does not exist", sales_dir);
        return Ok(());
    }

    let mut output_writer = Writer::from_path(output_file)?;
    let mut headers_written = false;

    let entries = fs::read_dir(sales_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            println!("Processing sales file: {}", path.display());
            
            let mut reader = Reader::from_reader(fs::File::open(&path)?);
            reader.headers()?; // Force reading headers
            
            if !headers_written {
                let headers = reader.headers()?;
                output_writer.write_record(headers)?;
                headers_written = true;
            }
            
            for result in reader.records() {
                let record = result?;
                output_writer.write_record(&record)?;
            }
        }
    }

    output_writer.flush()?;
    println!("Successfully merged sales files into {}", output_file);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    process_invoices()?;
    process_payments()?;
    process_sales()?;
    Ok(())
}
