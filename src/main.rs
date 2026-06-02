use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::Path;
use regex::Regex;

fn main() {
    // 1. Gather command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [--verbose] <path-to-pdf-file>", args[0]);
        std::process::exit(1);
    }
    let mut verbose = false;
    if args.len() > 2 && args[1] == "--verbose" {
        verbose = true;
    }
    let pdf_path = &args[args.len() - 1];

    if !Path::new(pdf_path).exists() {
        eprintln!("Error: File '{}' does not exist.", pdf_path);
        std::process::exit(1);
    }

    println!("Extracting text from {}...", pdf_path);

    // 2. Extract text from the PDF file
    let text = match pdf_extract::extract_text_from_mem(&std::fs::read(pdf_path).unwrap()) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error extracting text from PDF: {}", e);
            std::process::exit(1);
        }
    };

    // 3. Regex to match numerical sequences
    // Matches digits, accounting for commas/periods as decimal or thousands separators
    let num_regex = Regex::new(r"\b\d[\d,.]*\b").unwrap();
    
    let mut counts: HashMap<char, usize> = HashMap::new();
    let mut total_valid_numbers = 0;

    // 4. Parse numbers and identify leading significant digits (1-9)
    if verbose {
        println!("Found numbers:");
    }
    for mat in num_regex.find_iter(&text) {
        let num_str = mat.as_str();
        if verbose {
            println!("{num_str}");
        }
        // Find the first non-zero, non-punctuation digit
        if let Some(leading_digit) = num_str.chars().find(|c| c.is_ascii_digit() && *c != '0') {
            *counts.entry(leading_digit).or_insert(0) += 1;
            total_valid_numbers += 1;
        }
    }

    if total_valid_numbers <= 0 {
        println!("No valid numbers with a leading digit of 1-9 were found in the PDF.");
        return;
    }

    // 5. Calculate Benford's Law Expected Percentages
    // P(d) = log10(1 + 1/d)
    let mut benford_pcts = HashMap::new();
    for d in 1..=9 {
        let digit_char = char::from_digit(d, 10).unwrap();
        let pct = ((1.0 + 1.0 / (d as f64)).log10()) * 100.0;
        benford_pcts.insert(digit_char, pct);
    }

    // 6. Render Results to Terminal
    println!("\n=== Analysis Complete ===");
    println!("Total numbers analyzed: {}\n", total_valid_numbers);

    println!("Digit | Observed Freq | Benford Freq  | Distribution Comparison (Observed: #, Benford: *)");
    println!("------+---------------+---------------+---------------------------------------------------");

    for d in 1..=9 {
        let digit_char = char::from_digit(d, 10).unwrap();
        
        // Observed calculation
        let observed_count = *counts.get(&digit_char).unwrap_or(&0);
        let observed_pct = (observed_count as f64 / total_valid_numbers as f64) * 100.0;
        
        // Expected calculation
        let benford_pct = benford_pcts[&digit_char];

        // Generate ASCII bar charts (scale: 1 character per 1.5%)
        let scale = 1.5;
        let observed_bar_len = (observed_pct / scale).round() as usize;
        let benford_bar_len = (benford_pct / scale).round() as usize;

        let observed_bar = "#".repeat(observed_bar_len);
        let benford_bar = "*".repeat(benford_bar_len);

        println!(
            "  {}   |  {:>5.1}% ({:>4}) |   {:>5.1}%     | Obs: {}",
            digit_char, observed_pct, observed_count, benford_pct, observed_bar
        );
        println!(
            "      |               |               | Ben: {}",
            benford_bar
        );
        println!("------+---------------+---------------+---------------------------------------------------");
    }
}
