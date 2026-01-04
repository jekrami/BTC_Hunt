/// Daily report generator for BTC Hunt
/// Reads statistics logs and generates summary reports with email notifications

use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Default)]
struct DailyStats {
    batches: u64,
    mnemonics: u64,
    addresses: u64,
    matches_found: u64,
    runtime_seconds: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    let stats_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("stats")
    };

    let email = if args.len() > 2 {
        Some(args[2].clone())
    } else {
        None
    };

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║            BTC Hunt - Daily Report Generator              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Get yesterday's date
    let yesterday = chrono::Local::now() - chrono::Duration::days(1);
    let date_str = yesterday.format("%Y-%m-%d").to_string();
    
    println!("Generating report for: {}\n", date_str);

    // Create reports directory
    let report_dir = PathBuf::from("reports").join(&date_str);
    fs::create_dir_all(&report_dir)?;

    let summary_file = report_dir.join("summary.txt");
    let mut summary = File::create(&summary_file)?;

    // Write header
    writeln!(summary, "╔════════════════════════════════════════════════════════════╗")?;
    writeln!(summary, "║          BTC Hunt Daily Report - {}          ║", date_str)?;
    writeln!(summary, "╚════════════════════════════════════════════════════════════╝")?;
    writeln!(summary)?;
    writeln!(summary, "Generated: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(summary)?;

    // Parse statistics from log files
    let mut total_stats = DailyStats::default();
    let mut log_files = Vec::new();

    if stats_dir.exists() {
        for entry in fs::read_dir(&stats_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("log") {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    if file_name.starts_with(&date_str.replace("-", "")) {
                        log_files.push(path);
                    }
                }
            }
        }
    }

    println!("Found {} log files for {}", log_files.len(), date_str);

    for log_file in &log_files {
        if let Ok(stats) = parse_log_file(log_file) {
            total_stats.batches += stats.batches;
            total_stats.mnemonics += stats.mnemonics;
            total_stats.addresses += stats.addresses;
            total_stats.matches_found += stats.matches_found;
            total_stats.runtime_seconds += stats.runtime_seconds;
        }
    }

    // Check for match reports
    let results_dir = PathBuf::from("results");
    let mut match_files = Vec::new();
    if results_dir.exists() {
        for entry in fs::read_dir(&results_dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if file_name.starts_with("MATCH_FOUND_") {
                    // Check if file is from yesterday (timestamp in filename)
                    match_files.push(path);
                }
            }
        }
    }

    // Write statistics
    writeln!(summary, "═══════════════════════════════════════════════════════════")?;
    writeln!(summary, "STATISTICS")?;
    writeln!(summary, "═══════════════════════════════════════════════════════════")?;
    writeln!(summary)?;
    writeln!(summary, "Batches Processed:     {}", total_stats.batches)?;
    writeln!(summary, "Mnemonics Generated:   {}", total_stats.mnemonics)?;
    writeln!(summary, "Addresses Derived:     {}", total_stats.addresses)?;
    writeln!(summary, "Addresses Checked:     {}", total_stats.addresses)?;
    writeln!(summary)?;
    
    if total_stats.runtime_seconds > 0 {
        let hours = total_stats.runtime_seconds / 3600;
        let minutes = (total_stats.runtime_seconds % 3600) / 60;
        writeln!(summary, "Total Runtime:         {}h {}m", hours, minutes)?;
        
        if total_stats.mnemonics > 0 {
            let rate = total_stats.mnemonics / total_stats.runtime_seconds.max(1);
            writeln!(summary, "Average Rate:          {} mnemonics/sec", rate)?;
        }
    }
    writeln!(summary)?;

    // Write match information
    writeln!(summary, "═══════════════════════════════════════════════════════════")?;
    writeln!(summary, "MATCHES FOUND")?;
    writeln!(summary, "═══════════════════════════════════════════════════════════")?;
    writeln!(summary)?;
    
    if match_files.is_empty() {
        writeln!(summary, "No matches found on {}", date_str)?;
    } else {
        writeln!(summary, "🎉 {} MATCH(ES) FOUND!", match_files.len())?;
        writeln!(summary)?;
        for match_file in &match_files {
            writeln!(summary, "  • {}", match_file.display())?;
        }
    }
    writeln!(summary)?;

    // Performance metrics
    writeln!(summary, "═══════════════════════════════════════════════════════════")?;
    writeln!(summary, "PERFORMANCE")?;
    writeln!(summary, "═══════════════════════════════════════════════════════════")?;
    writeln!(summary)?;
    
    if total_stats.addresses > 0 && total_stats.runtime_seconds > 0 {
        let addr_per_sec = total_stats.addresses / total_stats.runtime_seconds;
        writeln!(summary, "Addresses per second:  {}", addr_per_sec)?;
        
        let searches_per_hour = addr_per_sec * 3600;
        writeln!(summary, "Searches per hour:     {}", searches_per_hour)?;
        
        let searches_per_day = searches_per_hour * 24;
        writeln!(summary, "Searches per day:      {}", searches_per_day)?;
    }
    writeln!(summary)?;

    // Disk usage
    writeln!(summary, "═══════════════════════════════════════════════════════════")?;
    writeln!(summary, "SYSTEM INFO")?;
    writeln!(summary, "═══════════════════════════════════════════════════════════")?;
    writeln!(summary)?;
    writeln!(summary, "Log files:             {}", log_files.len())?;
    writeln!(summary, "Report location:       {}", summary_file.display())?;
    writeln!(summary)?;

    println!("\n✓ Report generated: {}", summary_file.display());

    // Send email if matches found and email provided
    if !match_files.is_empty() && email.is_some() {
        send_email_notification(&email.unwrap(), &date_str, &match_files)?;
    }

    // Print summary to console
    println!("\n═══════════════════════════════════════════════════════════");
    println!("SUMMARY FOR {}", date_str);
    println!("═══════════════════════════════════════════════════════════");
    println!("Mnemonics:     {}", total_stats.mnemonics);
    println!("Addresses:     {}", total_stats.addresses);
    println!("Matches:       {}", match_files.len());
    println!("Runtime:       {}s", total_stats.runtime_seconds);
    println!("═══════════════════════════════════════════════════════════\n");

    Ok(())
}

fn parse_log_file(path: &Path) -> Result<DailyStats, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut stats = DailyStats::default();

    for line in reader.lines() {
        let line = line?;
        
        // Parse statistics lines
        if line.contains("Batches:") {
            if let Some(num) = extract_number(&line) {
                stats.batches = num;
            }
        } else if line.contains("Mnemonics:") {
            if let Some(num) = extract_number(&line) {
                stats.mnemonics = num;
            }
        } else if line.contains("Addresses:") {
            if let Some(num) = extract_number(&line) {
                stats.addresses = num;
            }
        } else if line.contains("Runtime:") {
            if let Some(num) = extract_number(&line) {
                stats.runtime_seconds = num;
            }
        } else if line.contains("MATCH FOUND") {
            stats.matches_found += 1;
        }
    }

    Ok(stats)
}

fn extract_number(line: &str) -> Option<u64> {
    line.split_whitespace()
        .find_map(|s| s.trim_matches(|c: char| !c.is_numeric()).parse().ok())
}

fn send_email_notification(email: &str, date: &str, match_files: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
    let subject = format!("BTC Hunt - MATCH FOUND on {}", date);
    let mut body = format!("🎉 Bitcoin Address Match Found!\n\nDate: {}\n\nMatches:\n", date);
    
    for file in match_files {
        body.push_str(&format!("  • {}\n", file.display()));
    }

    // Try to use mail command (Unix/Linux)
    #[cfg(unix)]
    {
        use std::process::Command;
        
        let output = Command::new("mail")
            .arg("-s")
            .arg(&subject)
            .arg(email)
            .stdin(std::process::Stdio::piped())
            .spawn()?
            .stdin
            .unwrap()
            .write_all(body.as_bytes())?;
        
        println!("✓ Email sent to: {}", email);
    }

    // Windows: Use PowerShell Send-MailMessage
    #[cfg(windows)]
    {
        println!("⚠ Email notification on Windows requires SMTP configuration.");
        println!("  Run this PowerShell command manually:");
        println!("\n  Send-MailMessage -To \"{}\" -Subject \"{}\" -Body \"{}\" -SmtpServer \"your-smtp-server\" -From \"btc-hunt@localhost\"", email, subject, body);
    }

    Ok(())
}



