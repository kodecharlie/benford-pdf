use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::path::Path;

struct DigitAnalysis {
    digit: char,
    observed_count: usize,
    observed_pct: f64,
    expected_count: f64,
    expected_pct: f64,
    z_score: f64,
    flagged: bool,
}

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

    if total_valid_numbers == 0 {
        println!("No valid numbers with a leading digit of 1-9 were found in the PDF.");
        return;
    }

    let n = total_valid_numbers as f64;

    // 5. Compute Expected Frequencies, Chi-Square, and Z-Scores
    let mut chi_square = 0.0;
    let mut digit_analyses: Vec<DigitAnalysis> = Vec::new();

    // Critical Z-score for alpha = 0.05 (two-tailed test)
    let z_critical = 1.96;

    for d in 1..=9 {
        let digit_char = char::from_digit(d, 10).unwrap();
        let obs_count = *counts.get(&digit_char).unwrap_or(&0);
        let obs_pct = (obs_count as f64 / n) * 100.0;

        // Expected probability P(d) = log10(1 + 1/d)
        let exp_prob = (1.0 + 1.0 / (d as f64)).log10();
        let exp_pct = exp_prob * 100.0;
        let exp_count = exp_prob * n;

        // Contribution to Chi-Square: (O - E)^2 / E
        let diff = obs_count as f64 - exp_count;
        chi_square += (diff * diff) / exp_count;

        // Z-score calculation with continuity correction (1 / 2n)
        let obs_prop = obs_count as f64 / n;
        let std_dev = (exp_prob * (1.0 - exp_prob) / n).sqrt();
        let continuity_correction = 0.5 / n;
        
        let z_score = if std_dev > 0.0 {
            let numerator = (obs_prop - exp_prob).abs() - continuity_correction;
            if numerator > 0.0 {
                (numerator / std_dev) * (obs_prop - exp_prob).signum()
            } else {
                0.0
            }
        } else {
            0.0
        };

        let flagged = z_score.abs() > z_critical;

        digit_analyses.push(DigitAnalysis {
            digit: digit_char,
            observed_count: obs_count,
            observed_pct: obs_pct,
            expected_count: exp_count,
            expected_pct: exp_pct,
            z_score,
            flagged,
        });
    }

    // 6. Render Terminal Results
    println!("\n=== Analysis Complete ===");
    println!("Total numbers analyzed: {}\n", total_valid_numbers);

    // Distribution Comparison Table
    println!("Digit | Observed Freq | Benford Freq  | Distribution Comparison (Observed: #, Benford: *)");
    println!("------+---------------+---------------+---------------------------------------------------");

    for analysis in &digit_analyses {
        let scale = 1.5;
        let observed_bar_len = (analysis.observed_pct / scale).round() as usize;
        let benford_bar_len = (analysis.expected_pct / scale).round() as usize;

        let observed_bar = "#".repeat(observed_bar_len);
        let benford_bar = "*".repeat(benford_bar_len);

        println!(
            "  {}   |  {:>5.1}% ({:>4}) |   {:>5.1}%     | Obs: {}",
            analysis.digit, analysis.observed_pct, analysis.observed_count, analysis.expected_pct, observed_bar
        );
        println!(
            "      |               |               | Ben: {}",
            benford_bar
        );
        println!("------+---------------+---------------+---------------------------------------------------");
    }

    // Calculate Nigrini's Mean Absolute Deviation (MAD)
    let mut total_abs_dev = 0.0;
    for d in 1..=9 {
        let digit_char = char::from_digit(d, 10).unwrap();
        let obs_count = *counts.get(&digit_char).unwrap_or(&0);
        let obs_prop = obs_count as f64 / n;
        let exp_prob = (1.0 + 1.0 / (d as f64)).log10();
        
        total_abs_dev += (obs_prop - exp_prob).abs();
    }
    let mad = total_abs_dev / 9.0;

    println!("\n=== Nigrini's Mean Absolute Deviation (MAD) ===");
    println!("MAD Score: {:.5}", mad);
    if mad <= 0.006 {
        println!("Conformity: Close Conformity");
    } else if mad <= 0.012 {
        println!("Conformity: Acceptable Conformity");
    } else if mad <= 0.015 {
        println!("Conformity: Marginally Non-Conforming");
    } else {
        println!("Conformity: Non-Conforming");
    }

    // 7. Overall Chi-Square Statistic Output
    let chi_critical = 15.51;
    let chi_exceeded = chi_square > chi_critical;

    println!("\n=== Chi-Square Goodness-of-Fit Test ===");
    println!("Chi-Square Statistic (\u{03C7}\u{00B2}): {:.4}", chi_square);
    println!("Critical Threshold (\u{03B1} = 0.05, df = 8): {:.2}", chi_critical);
    if chi_exceeded {
        println!("Result: EXCEEDS threshold ({:.4} > {:.2}). The dataset significantly deviates from Benford's Law.", chi_square, chi_critical);
    } else {
        println!("Result: DOES NOT exceed threshold ({:.4} <= {:.2}). The dataset conforms to Benford's Law.", chi_square, chi_critical);
    }

    // 8. Individual Digit Z-Score Table (Sorted highest to lowest Z-Score magnitude)
    let mut sorted_analyses = digit_analyses;
    sorted_analyses.sort_by(|a, b| b.z_score.abs().partial_cmp(&a.z_score.abs()).unwrap());

    println!("\n=== Individual Digit Z-Score Analysis ===");
    println!("Sorted by absolute deviation magnitude (Critical |Z| = 1.96, \u{03B1} = 0.05)\n");
    println!(" Digit | Obs Count | Exp Count | Obs Pct | Exp Pct |  Z-Score  | Flagged for Review ");
    println!("-------+-----------+-----------+---------+---------+-----------+--------------------");

    for a in &sorted_analyses {
        let flag_str = if a.flagged {
            "YES [FLAGGED]"
        } else {
            "No"
        };
        println!(
            "   {}   |   {:>6}  |  {:>8.1} |  {:>5.1}% |  {:>5.1}% |  {:>+8.3}  | {}",
            a.digit, a.observed_count, a.expected_count, a.observed_pct, a.expected_pct, a.z_score, flag_str
        );
    }
    println!("------------------------------------------------------------------------------------\n");
}
