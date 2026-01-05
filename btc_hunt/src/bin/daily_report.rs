/// Daily report generator for BTC Hunt
/// Reads statistics logs and generates summary reports with email notifications

use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Default, Debug)]
struct DailyStats {
    batches: u64,
    mnemonics: u64,
    addresses: u64,
    checked: u64,
    matches_found: u64,
    runtime_seconds: u64,
    hostname: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    let mut stats_dir = PathBuf::from(".");
    let mut email: Option<String> = None;
    
    // Parse command line arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--log-dir" => {
                if i + 1 < args.len() {
                    stats_dir = PathBuf::from(&args[i + 1]);
                    i += 2;
                } else {
                    eprintln!("Error: --log-dir requires a path");
                    std::process::exit(1);
                }
            }
            "--email" => {
                if i + 1 < args.len() {
                    email = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --email requires an email address");
                    std::process::exit(1);
                }
            }
            _ => {
                // Legacy support: first arg is stats_dir, second is email
                if i == 1 {
                    stats_dir = PathBuf::from(&args[i]);
                } else if i == 2 {
                    email = Some(args[i].clone());
                }
                i += 1;
            }
        }
    }
    
    // Get hostname
    let hostname = get_hostname();

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║            BTC Hunt - Daily Report Generator              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Get yesterday's date
    let yesterday = chrono::Local::now() - chrono::Duration::days(1);
    let date_str = yesterday.format("%Y-%m-%d").to_string();
    
    println!("Generating report for: {}\n", date_str);
    println!("Log directory: {}\n", stats_dir.display());

    // Create reports directory
    let report_dir = PathBuf::from("reports");
    fs::create_dir_all(&report_dir)?;

    let summary_file = report_dir.join(format!("report_{}.txt", date_str));
    let mut summary = File::create(&summary_file)?;

    // Write header
    writeln!(summary, "╔════════════════════════════════════════════════════════════╗")?;
    writeln!(summary, "║          BTC Hunt Daily Report - {}          ║", date_str)?;
    writeln!(summary, "╚════════════════════════════════════════════════════════════╝")?;
    writeln!(summary)?;
    writeln!(summary, "Generated: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(summary, "Server:    {}", hostname)?;
    writeln!(summary)?;

    // Find log file for yesterday (date format: YYYYMMDD)
    let log_date_str = date_str.replace("-", ""); // 20260104
    let log_file = stats_dir.join(format!("btc_hunt_{}.log", log_date_str));
    
    if !log_file.exists() {
        eprintln!("Warning: Log file not found: {}", log_file.display());
        writeln!(summary, "ERROR: No log file found at {}", log_file.display())?;
        writeln!(summary, "Expected: btc_hunt_{}.log in {}", log_date_str, stats_dir.display())?;
        return Ok(());
    }

    println!("Reading log file: {}", log_file.display());

    // Parse statistics from LAST line of log file (contains final daily totals)
    let mut total_stats = DailyStats::default();
    total_stats.hostname = hostname.clone();
    
    if let Ok(stats) = parse_last_log_line(&log_file) {
        total_stats = stats;
        total_stats.hostname = hostname.clone();
    } else {
        println!("Warning: Could not parse stats from {}", log_file.display());
        writeln!(summary, "ERROR: Could not parse log file")?;
    }

    // Check for match reports from yesterday
    let results_dir = PathBuf::from("results");
    let mut match_files = Vec::new();
    if results_dir.exists() {
        for entry in fs::read_dir(&results_dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if file_name.starts_with("MATCH_FOUND_") && file_name.contains(&date_str.replace("-", "")) {
                    match_files.push(path);
                }
            }
        }
    }
    
    total_stats.matches_found = match_files.len() as u64;

    // Write statistics
    writeln!(summary, "═══════════════════════════════════════════════════════════")?;
    writeln!(summary, "DAILY STATISTICS")?;
    writeln!(summary, "═══════════════════════════════════════════════════════════")?;
    writeln!(summary)?;
    writeln!(summary, "Date:                  {}", date_str)?;
    writeln!(summary, "Server:                {}", total_stats.hostname)?;
    writeln!(summary)?;
    writeln!(summary, "Batches Processed:     {}", total_stats.batches)?;
    writeln!(summary, "Mnemonics Generated:   {}", total_stats.mnemonics)?;
    writeln!(summary, "Addresses Derived:     {}", total_stats.addresses)?;
    writeln!(summary, "Addresses Checked:     {}", total_stats.checked)?;
    writeln!(summary)?;
    
    if total_stats.runtime_seconds > 0 {
        let hours = total_stats.runtime_seconds / 3600;
        let minutes = (total_stats.runtime_seconds % 3600) / 60;
        let seconds = total_stats.runtime_seconds % 60;
        writeln!(summary, "Total Runtime:         {}h {}m {}s", hours, minutes, seconds)?;
        
        if total_stats.checked > 0 {
            let rate = total_stats.checked / total_stats.runtime_seconds.max(1);
            writeln!(summary, "Check Rate:            {} addresses/sec", rate)?;
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
    
    if total_stats.checked > 0 && total_stats.runtime_seconds > 0 {
        let addr_per_sec = total_stats.checked / total_stats.runtime_seconds;
        writeln!(summary, "Addresses per second:  {}", addr_per_sec)?;
        
        let searches_per_hour = addr_per_sec * 3600;
        writeln!(summary, "Searches per hour:     {}", searches_per_hour)?;
        
        let searches_per_day = searches_per_hour * 24;
        writeln!(summary, "Searches per day:      {}", searches_per_day)?;
    }
    writeln!(summary)?;

    println!("\n✓ Report generated: {}", summary_file.display());

    // Send email if email provided (ALWAYS send daily report)
    if let Some(email_addr) = email {
        send_daily_email(&email_addr, &date_str, &total_stats, &match_files)?;
    }

    // Print summary to console
    println!("\n═══════════════════════════════════════════════════════════");
    println!("SUMMARY FOR {} - {}", date_str, total_stats.hostname);
    println!("═══════════════════════════════════════════════════════════");
    println!("Batches:       {}", total_stats.batches);
    println!("Mnemonics:     {}", total_stats.mnemonics);
    println!("Addresses:     {}", total_stats.addresses);
    println!("Checked:       {}", total_stats.checked);
    println!("Matches:       {}", match_files.len());
    println!("Runtime:       {}s", total_stats.runtime_seconds);
    println!("═══════════════════════════════════════════════════════════\n");

    Ok(())
}

fn get_hostname() -> String {
    #[cfg(unix)]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("hostname").output() {
            if let Ok(hostname) = String::from_utf8(output.stdout) {
                return hostname.trim().to_string();
            }
        }
    }
    
    #[cfg(windows)]
    {
        if let Ok(hostname) = std::env::var("COMPUTERNAME") {
            return hostname;
        }
    }
    
    "unknown".to_string()
}

fn parse_last_log_line(path: &Path) -> Result<DailyStats, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut stats = DailyStats::default();
    let mut last_stats_line: Option<String> = None;

    // Read all lines and keep track of the last stats line
    // Format: [2026-01-04 17:17:14] Runtime: 10 | Batches: 0 | Mnemonics: 10000 | Addresses: 0 | Checked: 0
    for line in reader.lines() {
        let line = line?;
        
        // Count matches
        if line.contains("MATCH FOUND") || line.contains("✓ Found") {
            stats.matches_found += 1;
        }
        
        // Check if it's a stats line (not a match notification)
        if line.contains("Runtime:") && line.contains("Batches:") {
            last_stats_line = Some(line);
        }
    }

    // Parse the last stats line
    if let Some(line) = last_stats_line {
        // Parse the fields
        if let Some(stats_part) = line.split("] ").nth(1) {
            let fields: Vec<&str> = stats_part.split(" | ").collect();
            
            for field in fields {
                let parts: Vec<&str> = field.split(": ").collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    if let Ok(value) = parts[1].trim().parse::<u64>() {
                        match key {
                            "Runtime" => stats.runtime_seconds = value,
                            "Batches" => stats.batches = value,
                            "Mnemonics" => stats.mnemonics = value,
                            "Addresses" => stats.addresses = value,
                            "Checked" => stats.checked = value,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(stats)
}

fn send_daily_email(email: &str, date: &str, stats: &DailyStats, match_files: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
    let subject = if !match_files.is_empty() {
        format!("BTC Hunt Daily Report - {} - {} MATCH(ES) FOUND! 🎉", date, match_files.len())
    } else {
        format!("BTC Hunt Daily Report - {}", date)
    };
    
    let mut body = format!(
        "╔════════════════════════════════════════════════════════════╗\n\
         ║          BTC Hunt Daily Report - {}          ║\n\
         ╚════════════════════════════════════════════════════════════╝\n\n",
        date
    );
    
    body.push_str(&format!("Date:                  {}\n", date));
    body.push_str(&format!("Server:                {}\n\n", stats.hostname));
    
    body.push_str("═══════════════════════════════════════════════════════════\n");
    body.push_str("DAILY STATISTICS\n");
    body.push_str("═══════════════════════════════════════════════════════════\n\n");
    
    body.push_str(&format!("Batches Processed:     {}\n", stats.batches));
    body.push_str(&format!("Mnemonics Generated:   {}\n", stats.mnemonics));
    body.push_str(&format!("Addresses Derived:     {}\n", stats.addresses));
    body.push_str(&format!("Addresses Checked:     {}\n", stats.checked));
    
    if stats.runtime_seconds > 0 {
        let hours = stats.runtime_seconds / 3600;
        let minutes = (stats.runtime_seconds % 3600) / 60;
        let seconds = stats.runtime_seconds % 60;
        body.push_str(&format!("\nTotal Runtime:         {}h {}m {}s\n", hours, minutes, seconds));
        
        if stats.checked > 0 {
            let rate = stats.checked / stats.runtime_seconds.max(1);
            body.push_str(&format!("Check Rate:            {} addresses/sec\n", rate));
        }
    }
    
    body.push_str("\n═══════════════════════════════════════════════════════════\n");
    body.push_str("MATCHES FOUND\n");
    body.push_str("═══════════════════════════════════════════════════════════\n\n");
    
    if match_files.is_empty() {
        body.push_str("No matches found today.\n");
    } else {
        body.push_str(&format!("🎉 {} MATCH(ES) FOUND!\n\n", match_files.len()));
        for file in match_files {
            body.push_str(&format!("  • {}\n", file.display()));
        }
    }
    
    body.push_str("\n---\n");
    body.push_str("BTC Hunt - Automated Daily Report\n");

    // Try to use mail command (Unix/Linux)
    #[cfg(unix)]
    {
        use std::process::Command;
        
        Command::new("mail")
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



