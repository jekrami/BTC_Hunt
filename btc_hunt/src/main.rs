use bip39::Mnemonic;
use bitcoin::address::Address;
use bitcoin::bip32::{DerivationPath, ExtendedPrivKey};
use bitcoin::key::Secp256k1;
use bitcoin::network::constants::Network;
use bitcoin::PrivateKey;
use clap::Parser;
use hex;
use rand::{rngs::OsRng, RngCore};
use rayon::prelude::*;
use rusqlite::{Connection, OpenFlags};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Bitcoin Address Hunter - Unified BIP39 generator, address deriver, and database checker
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// SQLite database path containing Bitcoin addresses
    #[arg(short, long, default_value = "btc_addresses.db")]
    database: PathBuf,

    /// Number of mnemonics to generate per batch
    #[arg(short = 'b', long, default_value_t = 1000)]
    batch_size: usize,

    /// Number of batches to process (0 = unlimited)
    #[arg(short = 'n', long, default_value_t = 0)]
    max_batches: usize,

    /// Number of addresses to derive per derivation path
    #[arg(short = 'a', long, default_value_t = 10)]
    addresses_per_path: u32,

    /// Output directory for results when match is found
    #[arg(short, long, default_value = "results")]
    output_dir: PathBuf,

    /// Number of worker threads (0 = auto-detect)
    #[arg(short = 't', long, default_value_t = 0)]
    threads: usize,

    /// Statistics update interval in seconds
    #[arg(short, long, default_value_t = 10)]
    stats_interval: u64,

    /// Batch size for database queries
    #[arg(long, default_value_t = 1000)]
    db_batch_size: usize,

    /// Log statistics to file for daily reports
    #[arg(long)]
    log_stats: bool,

    /// Email address for match notifications (requires mail command)
    #[arg(long)]
    email: Option<String>,

    /// Simple output mode for servers/logging (no fancy formatting)
    #[arg(long)]
    simple: bool,
}

/// Log file manager with daily rotation
struct LogManager {
    stats_dir: PathBuf,
    current_date: Mutex<String>,
    file: Mutex<Option<File>>,
}

impl LogManager {
    fn new(stats_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        std::fs::create_dir_all(&stats_dir)?;
        let today = chrono::Local::now().format("%Y%m%d").to_string();
        let log_path = stats_dir.join(format!("btc_hunt_{}.log", today));
        let file = File::create(log_path)?;
        
        Ok(Self {
            stats_dir,
            current_date: Mutex::new(today),
            file: Mutex::new(Some(file)),
        })
    }
    
    fn write_log(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let today = chrono::Local::now().format("%Y%m%d").to_string();
        let mut current_date = self.current_date.lock().unwrap();
        let mut file_opt = self.file.lock().unwrap();
        
        // Check if day has changed
        if *current_date != today {
            // Close old file and create new one
            *file_opt = None;
            let log_path = self.stats_dir.join(format!("btc_hunt_{}.log", today));
            *file_opt = Some(File::create(log_path)?);
            *current_date = today.clone();
        }
        
        // Write to file
        if let Some(ref mut file) = *file_opt {
            writeln!(file, "{}", message)?;
            file.flush()?;
        }
        
        Ok(())
    }
}

/// Statistics tracking
struct Stats {
    mnemonics_generated: AtomicU64,
    addresses_derived: AtomicU64,
    addresses_checked: AtomicU64,
    batches_processed: AtomicU64,
    start_time: Instant,
    current_date: Mutex<String>,
}

impl Stats {
    fn new() -> Self {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        Self {
            mnemonics_generated: AtomicU64::new(0),
            addresses_derived: AtomicU64::new(0),
            addresses_checked: AtomicU64::new(0),
            batches_processed: AtomicU64::new(0),
            start_time: Instant::now(),
            current_date: Mutex::new(today),
        }
    }
    
    fn check_and_reset_if_new_day(&self) -> bool {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut current_date = self.current_date.lock().unwrap();
        
        if *current_date != today {
            // New day! Reset counters
            self.mnemonics_generated.store(0, Ordering::Relaxed);
            self.addresses_derived.store(0, Ordering::Relaxed);
            self.addresses_checked.store(0, Ordering::Relaxed);
            self.batches_processed.store(0, Ordering::Relaxed);
            *current_date = today;
            true
        } else {
            false
        }
    }

    fn print_stats(&self, simple_mode: bool) {
        let elapsed = self.start_time.elapsed().as_secs();
        let mnemonics = self.mnemonics_generated.load(Ordering::Relaxed);
        let addresses = self.addresses_derived.load(Ordering::Relaxed);
        let checked = self.addresses_checked.load(Ordering::Relaxed);
        let batches = self.batches_processed.load(Ordering::Relaxed);

        let mnemonic_rate = if elapsed > 0 { mnemonics / elapsed } else { 0 };
        let address_rate = if elapsed > 0 { addresses / elapsed } else { 0 };

        if simple_mode {
            // Simple, log-friendly output
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            eprintln!("[{}] Checked: {} addresses | Mnemonics: {} | Batches: {} | Runtime: {}s | Rate: {}/s",
                timestamp, checked, mnemonics, batches, elapsed, address_rate);
        } else {
            // Fancy formatted output
            eprintln!("\n╔════════════════════════════════════════════════════════════╗");
            eprintln!("║                    BTC HUNT STATISTICS                     ║");
            eprintln!("╠════════════════════════════════════════════════════════════╣");
            eprintln!("║ Runtime:         {} seconds", elapsed);
            eprintln!("║ Batches:         {}", batches);
            eprintln!("║ Mnemonics:       {} ({}/s)", mnemonics, mnemonic_rate);
            eprintln!("║ Addresses:       {} ({}/s)", addresses, address_rate);
            eprintln!("║ Checked:         {}", checked);
            eprintln!("╚════════════════════════════════════════════════════════════╝");
        }
    }

    fn to_log_string(&self) -> String {
        let elapsed = self.start_time.elapsed().as_secs();
        let mnemonics = self.mnemonics_generated.load(Ordering::Relaxed);
        let addresses = self.addresses_derived.load(Ordering::Relaxed);
        let checked = self.addresses_checked.load(Ordering::Relaxed);
        let batches = self.batches_processed.load(Ordering::Relaxed);

        format!(
            "Runtime: {} | Batches: {} | Mnemonics: {} | Addresses: {} | Checked: {}",
            elapsed, batches, mnemonics, addresses, checked
        )
    }
}

/// Represents a Bitcoin address with its derivation information
#[derive(Clone, Debug)]
struct DerivedAddress {
    address: String,
    path: String,
    private_key: String,
    public_key: String,
}

/// Result of processing a mnemonic
struct MnemonicResult {
    mnemonic: String,
    seed: Vec<u8>,
    addresses: Vec<DerivedAddress>,
}

/// BIP39 wordlist (embedded at compile time)
const WORDLIST: &str = include_str!("../bip39-english.txt");

lazy_static::lazy_static! {
    static ref WORDLIST_VEC: Vec<String> = WORDLIST.lines().map(String::from).collect();
}

/// Generate a BIP39 mnemonic phrase (12 words)
fn generate_mnemonic() -> Result<String, String> {
    const ENTROPY_BITS: usize = 128;
    const CHECKSUM_BITS: usize = ENTROPY_BITS / 32;
    const MNEMONIC_WORDS: usize = (ENTROPY_BITS + CHECKSUM_BITS) / 11;

    let mut entropy = vec![0u8; ENTROPY_BITS / 8];
    OsRng.fill_bytes(&mut entropy);

    // Calculate checksum
    let mut hasher = Sha256::new();
    hasher.update(&entropy);
    let hash = hasher.finalize();

    let mut checksum_bits = Vec::new();
    for i in 0..CHECKSUM_BITS {
        checksum_bits.push((hash[i / 8] >> (7 - (i % 8))) & 1 == 1);
    }

    // Combine entropy and checksum
    let mut entropy_bits = Vec::new();
    for byte in &entropy {
        for i in 0..8 {
            entropy_bits.push((byte >> (7 - i)) & 1 == 1);
        }
    }
    entropy_bits.extend(checksum_bits);

    // Convert to words
    let mut words = Vec::new();
    for i in 0..MNEMONIC_WORDS {
        let start = i * 11;
        let end = start + 11;
        let bits = &entropy_bits[start..end];
        let mut index = 0;
        for (j, bit) in bits.iter().enumerate() {
            if *bit {
                index |= 1 << (bits.len() - 1 - j);
            }
        }
        words.push(WORDLIST_VEC[index].clone());
    }

    Ok(words.join(" "))
}

/// Derive Bitcoin addresses from a mnemonic
fn derive_addresses(
    mnemonic_str: &str,
    addresses_per_path: u32,
) -> Result<MnemonicResult, String> {
    let secp = Secp256k1::new();

    // Parse mnemonic
    let mnemonic = Mnemonic::from_str(mnemonic_str)
        .map_err(|e| format!("Failed to parse mnemonic: {}", e))?;

    // Generate seed
    let seed = mnemonic.to_seed("");

    // Create master key
    let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)
        .map_err(|e| format!("Failed to create master key: {}", e))?;

    let mut addresses = Vec::new();

    // Define derivation paths with their properties
    // Format: (base_path, suffix_pattern, script_semantics)
    let derivation_configs = vec![
        ("m/0'/0'", "hardened", "P2PKH"),
        ("m/44'/0'/0'", "normal_hardened", "P2PKH"),
        ("m/49'/0'/0'", "normal_hardened", "P2WPKH nested in P2SH"),
        ("m/84'/0'/0'", "normal_hardened", "P2WPKH"),
        ("m/0", "hardened", "P2WPKH nested in P2SH"),
        ("m/0", "hardened", "P2WPKH"),
    ];

    for (base_path, suffix_pattern, script_semantics) in derivation_configs {
        for i in 0..addresses_per_path {
            // Build the path
            let path_str = match suffix_pattern {
                "hardened" => format!("{}/{}'", base_path, i),
                "normal_hardened" => format!("{}/0/{}'", base_path, i),
                _ => continue,
            };

            // Parse and derive
            let path = DerivationPath::from_str(&path_str)
                .map_err(|e| format!("Failed to parse path {}: {}", path_str, e))?;

            let child_key = master_key
                .derive_priv(&secp, &path)
                .map_err(|e| format!("Failed to derive key for path {}: {}", path_str, e))?;

            let private_key = PrivateKey::new(child_key.private_key, Network::Bitcoin);
            let public_key = private_key.public_key(&secp);

            // Generate address based on script type
            let address = match script_semantics {
                "P2PKH" => Address::p2pkh(&public_key, Network::Bitcoin),
                "P2WPKH nested in P2SH" => Address::p2shwpkh(&public_key, Network::Bitcoin)
                    .map_err(|e| format!("Failed to create P2WPKH nested address: {}", e))?,
                "P2WPKH" => Address::p2wpkh(&public_key, Network::Bitcoin)
                    .map_err(|e| format!("Failed to create P2WPKH address: {}", e))?,
                _ => continue,
            };

            addresses.push(DerivedAddress {
                address: address.to_string(),
                path: path_str,
                private_key: private_key.to_wif(),
                public_key: public_key.to_string(),
            });
        }
    }

    Ok(MnemonicResult {
        mnemonic: mnemonic_str.to_string(),
        seed: seed.to_vec(),
        addresses,
    })
}

/// Address match with balance information
#[derive(Clone, Debug)]
struct AddressMatch {
    address: String,
    balance: Option<f64>,
}

/// Check addresses against database
fn check_addresses_in_db(
    conn: &Connection,
    addresses: &[String],
    batch_size: usize,
) -> Result<Vec<AddressMatch>, Box<dyn std::error::Error>> {
    let mut found = Vec::new();

    for chunk in addresses.chunks(batch_size) {
        let placeholders = vec!["?"; chunk.len()].join(",");
        let query = format!(
            "SELECT address, balance FROM btc_addresses WHERE address IN ({})",
            placeholders
        );

        let mut stmt = conn.prepare_cached(&query)?;
        let params: Vec<&dyn rusqlite::ToSql> = chunk
            .iter()
            .map(|s| s as &dyn rusqlite::ToSql)
            .collect();

        let mut rows = stmt.query(&params[..])?;
        while let Some(row) = rows.next()? {
            let address: String = row.get(0)?;
            let balance: Option<f64> = row.get(1).ok();
            found.push(AddressMatch { address, balance });
        }
    }

    Ok(found)
}

/// Generate a detailed report when a match is found
fn generate_report(
    result: &MnemonicResult,
    found_addresses: &[AddressMatch],
    output_dir: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(output_dir)?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let report_file = output_dir.join(format!("MATCH_FOUND_{}.txt", timestamp));
    let mut file = File::create(&report_file)?;

    writeln!(file, "╔════════════════════════════════════════════════════════════╗")?;
    writeln!(file, "║                   BITCOIN ADDRESS MATCH FOUND!             ║")?;
    writeln!(file, "╚════════════════════════════════════════════════════════════╝")?;
    writeln!(file)?;
    writeln!(file, "Timestamp: {}", timestamp)?;
    let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    writeln!(file, "Date: {}", datetime)?;
    writeln!(file)?;
    writeln!(file, "BIP39 Mnemonic:")?;
    writeln!(file, "  {}", result.mnemonic)?;
    writeln!(file)?;
    writeln!(file, "BIP39 Seed:")?;
    writeln!(file, "  {}", hex::encode(&result.seed))?;
    writeln!(file)?;
    writeln!(file, "Found Addresses ({}):", found_addresses.len())?;
    for addr_match in found_addresses {
        if let Some(balance) = addr_match.balance {
            writeln!(file, "  ✓ {} (Balance: {} BTC)", addr_match.address, balance)?;
        } else {
            writeln!(file, "  ✓ {} (Balance: unknown)", addr_match.address)?;
        }
    }
    writeln!(file)?;
    writeln!(file, "═══════════════════════════════════════════════════════════")?;
    writeln!(file, "All Derived Addresses and Keys:")?;
    writeln!(file, "═══════════════════════════════════════════════════════════")?;
    writeln!(file)?;
    writeln!(file, "Path,Address,Public Key,Private Key")?;

    for addr in &result.addresses {
        writeln!(
            file,
            "{},{},{},{}",
            addr.path, addr.address, addr.public_key, addr.private_key
        )?;
    }

    eprintln!("\n✓ Report saved to: {}", report_file.display());
    Ok(())
}

/// Send email notification when match is found
fn send_email_notification(email: &str, addresses: &[AddressMatch]) -> Result<(), Box<dyn std::error::Error>> {
    let subject = format!("BTC Hunt - MATCH FOUND!");
    let addresses_text: String = addresses.iter().map(|a| {
        if let Some(balance) = a.balance {
            format!("  • {} (Balance: {} BTC)", a.address, balance)
        } else {
            format!("  • {} (Balance: unknown)", a.address)
        }
    }).collect::<Vec<_>>().join("\n");
    
    let body = format!(
        "🎉 Bitcoin Address Match Found!\n\nTimestamp: {}\n\nMatched Addresses ({}):\n{}\n\nCheck results/ directory for detailed report.",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        addresses.len(),
        addresses_text
    );

    eprintln!("\n📧 Sending email notification to: {}", email);

    // Try to use mail command (Unix/Linux)
    #[cfg(unix)]
    {
        use std::process::Command;
        
        let mut child = Command::new("mail")
            .arg("-s")
            .arg(&subject)
            .arg(email)
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(body.as_bytes())?;
        }
        
        child.wait()?;
        eprintln!("✓ Email sent!");
    }

    // Windows: Generate PowerShell script
    #[cfg(windows)]
    {
        eprintln!("⚠ Email on Windows requires PowerShell configuration.");
        eprintln!("\nTo send email, run this command:");
        eprintln!("\nSend-MailMessage `");
        eprintln!("  -To \"{}\" `", email);
        eprintln!("  -Subject \"{}\" `", subject);
        eprintln!("  -Body \"{}\" `", body.replace('\n', "`n"));
        eprintln!("  -SmtpServer \"smtp.gmail.com\" `");
        eprintln!("  -Port 587 `");
        eprintln!("  -UseSsl `");
        eprintln!("  -Credential (Get-Credential)");
        
        // Or create a PS1 file
        let ps_file = std::path::PathBuf::from("send_notification.ps1");
        let mut file = File::create(&ps_file)?;
        writeln!(file, "# BTC Hunt - Email Notification")?;
        writeln!(file, "$To = \"{}\"", email)?;
        writeln!(file, "$Subject = \"{}\"", subject)?;
        writeln!(file, "$Body = @\"\n{}\n\"@", body)?;
        writeln!(file, "\n# Configure your SMTP settings here")?;
        writeln!(file, "$SmtpServer = \"smtp.gmail.com\"  # Change to your SMTP server")?;
        writeln!(file, "$Port = 587")?;
        writeln!(file, "$Username = \"your-email@gmail.com\"  # Change this")?;
        writeln!(file, "\n# Uncomment to send:")?;
        writeln!(file, "# $Password = Read-Host -AsSecureString \"Enter password\"")?;
        writeln!(file, "# $Credential = New-Object System.Management.Automation.PSCredential($Username, $Password)")?;
        writeln!(file, "# Send-MailMessage -To $To -Subject $Subject -Body $Body -SmtpServer $SmtpServer -Port $Port -UseSsl -Credential $Credential -From $Username")?;
        
        eprintln!("\n✓ Email script saved to: {}", ps_file.display());
        eprintln!("  Edit the file with your SMTP settings and run it.");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Setup statistics logging if requested
    let log_manager = if args.log_stats {
        let stats_dir = std::path::PathBuf::from("stats");
        Some(Arc::new(LogManager::new(stats_dir)?))
    } else {
        None
    };

    // Set up thread pool
    if args.threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.threads)
            .build_global()
            .unwrap();
    }

    if !args.simple {
        eprintln!("╔════════════════════════════════════════════════════════════╗");
        eprintln!("║              Bitcoin Address Hunter v2.0                   ║");
        eprintln!("║          Unified Mnemonic → Address → Database             ║");
        eprintln!("╚════════════════════════════════════════════════════════════╝");
        eprintln!();
        eprintln!("Configuration:");
        eprintln!("  • Database:           {}", args.database.display());
        eprintln!("  • Batch size:         {} mnemonics", args.batch_size);
        eprintln!("  • Max batches:        {}", if args.max_batches == 0 { "unlimited".to_string() } else { args.max_batches.to_string() });
        eprintln!("  • Addresses/path:     {}", args.addresses_per_path);
        eprintln!("  • Worker threads:     {}", rayon::current_num_threads());
        eprintln!("  • DB batch size:      {}", args.db_batch_size);
        eprintln!();
    } else {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        eprintln!("[{}] BTC Hunt v2.0 starting | DB: {} | Batch: {} | Threads: {} | Log: {}", 
            timestamp, args.database.display(), args.batch_size, rayon::current_num_threads(),
            if args.log_stats { "enabled" } else { "disabled" });
    }

    // Open database connection
    let conn = Connection::open_with_flags(
        &args.database,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;

    // Set SQLite optimizations for read-only access
    conn.execute_batch(
        "PRAGMA temp_store = MEMORY;
         PRAGMA mmap_size = 268435456;
         PRAGMA cache_size = -64000;
         PRAGMA query_only = ON;
         PRAGMA read_uncommitted = 1;",
    )?;

    let stats = Arc::new(Stats::new());
    let found = Arc::new(AtomicBool::new(false));

    // Statistics printing thread
    let stats_clone = Arc::clone(&stats);
    let found_clone = Arc::clone(&found);
    let log_manager_clone = log_manager.clone();
    let stats_interval = args.stats_interval;
    let simple_mode = args.simple;
    std::thread::spawn(move || {
        while !found_clone.load(Ordering::Relaxed) {
            std::thread::sleep(Duration::from_secs(stats_interval));
            
            // Check for day change and reset if needed
            if stats_clone.check_and_reset_if_new_day() && !simple_mode {
                eprintln!("\n═══ New day started! Counters reset. ═══\n");
            }
            
            stats_clone.print_stats(simple_mode);
            
            // Log to file if enabled
            if let Some(ref log_mgr) = log_manager_clone {
                let log_line = format!("[{}] {}", 
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    stats_clone.to_log_string());
                let _ = log_mgr.write_log(&log_line);
            }
        }
    });

    if !args.simple {
        eprintln!("Starting hunt...\n");
    }

    let mut batch_count = 0;
    loop {
        if args.max_batches > 0 && batch_count >= args.max_batches {
            break;
        }

        // Generate mnemonics in parallel
        let mnemonics: Vec<String> = (0..args.batch_size)
            .into_par_iter()
            .filter_map(|_| generate_mnemonic().ok())
            .collect();

        stats
            .mnemonics_generated
            .fetch_add(mnemonics.len() as u64, Ordering::Relaxed);

        // Derive addresses in parallel
        let results: Vec<MnemonicResult> = mnemonics
            .par_iter()
            .filter_map(|mnemonic| derive_addresses(mnemonic, args.addresses_per_path).ok())
            .collect();

        // Collect all addresses for database check
        let mut address_to_result: HashMap<String, usize> = HashMap::new();
        let mut all_addresses = Vec::new();

        for (idx, result) in results.iter().enumerate() {
            for addr in &result.addresses {
                address_to_result.insert(addr.address.clone(), idx);
                all_addresses.push(addr.address.clone());
            }
        }

        stats
            .addresses_derived
            .fetch_add(all_addresses.len() as u64, Ordering::Relaxed);

        // Check addresses against database
        let found_addresses = check_addresses_in_db(&conn, &all_addresses, args.db_batch_size)?;

        stats
            .addresses_checked
            .fetch_add(all_addresses.len() as u64, Ordering::Relaxed);

        // If we found something, generate report and exit
        if !found_addresses.is_empty() {
            found.store(true, Ordering::Relaxed);
            
            // Find which mnemonic(s) produced the match
            let mut matched_results = HashSet::new();
            for found_addr in &found_addresses {
                if let Some(&idx) = address_to_result.get(&found_addr.address) {
                    matched_results.insert(idx);
                }
            }

            // Display match information
            for &idx in &matched_results {
                let result = &results[idx];
                let result_found_addrs: Vec<&AddressMatch> = found_addresses
                    .iter()
                    .filter(|addr| {
                        result.addresses.iter().any(|a| a.address == addr.address)
                    })
                    .collect();

                if args.simple {
                    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                    eprintln!("[{}] ╔══════════════════════════════════════════════════════════════╗", timestamp);
                    eprintln!("[{}] ║ *** MATCH FOUND! {} addresses matched ***", timestamp, result_found_addrs.len());
                    eprintln!("[{}] ╚══════════════════════════════════════════════════════════════╗", timestamp);
                    eprintln!("[{}] ", timestamp);
                    eprintln!("[{}] Mnemonic: {}", timestamp, result.mnemonic);
                    eprintln!("[{}] Seed: {}", timestamp, hex::encode(&result.seed));
                    eprintln!("[{}] ", timestamp);
                    eprintln!("[{}] Matched Addresses:", timestamp);
                    
                    for addr_match in &result_found_addrs {
                        // Find the full address info from result
                        if let Some(addr_info) = result.addresses.iter().find(|a| a.address == addr_match.address) {
                            let balance_str = if let Some(balance) = addr_match.balance {
                                format!("{} BTC", balance)
                            } else {
                                "unknown".to_string()
                            };
                            
                            eprintln!("[{}]   ✓ Address:     {}", timestamp, addr_match.address);
                            eprintln!("[{}]     Balance:     {}", timestamp, balance_str);
                            eprintln!("[{}]     Path:        {}", timestamp, addr_info.path);
                            eprintln!("[{}]     Public Key:  {}", timestamp, addr_info.public_key);
                            eprintln!("[{}]     Private Key: {}", timestamp, addr_info.private_key);
                            eprintln!("[{}] ", timestamp);
                        }
                    }
                    eprintln!("[{}] Full report saved to: results/MATCH_FOUND_*.txt", timestamp);
                    eprintln!("[{}] ════════════════════════════════════════════════════════════", timestamp);
                } else {
                    eprintln!("\n╔════════════════════════════════════════════════════════════╗");
                    eprintln!("║                   🎉 MATCH FOUND! 🎉                       ║");
                    eprintln!("╚════════════════════════════════════════════════════════════╝\n");
                    
                    eprintln!("BIP39 Mnemonic:");
                    eprintln!("  {}\n", result.mnemonic);
                    
                    eprintln!("BIP39 Seed:");
                    eprintln!("  {}\n", hex::encode(&result.seed));
                    
                    eprintln!("Matched Addresses ({}):", result_found_addrs.len());
                    for addr_match in &result_found_addrs {
                        // Find the full address info from result
                        if let Some(addr_info) = result.addresses.iter().find(|a| a.address == addr_match.address) {
                            let balance_str = if let Some(balance) = addr_match.balance {
                                format!("{} BTC", balance)
                            } else {
                                "unknown".to_string()
                            };
                            
                            eprintln!("  ════════════════════════════════════════════════════════════");
                            eprintln!("  ✓ Address:     {}", addr_match.address);
                            eprintln!("    Balance:     {}", balance_str);
                            eprintln!("    Path:        {}", addr_info.path);
                            eprintln!("    Public Key:  {}", addr_info.public_key);
                            eprintln!("    Private Key: {}", addr_info.private_key);
                        }
                    }
                    eprintln!("  ════════════════════════════════════════════════════════════\n");
                    eprintln!("Full report saved to: results/\n");
                }
            }

            // Log match found
            if let Some(ref log_mgr) = log_manager {
                let mut log_msg = format!("[{}] 🎉 MATCH FOUND! {} addresses matched",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    found_addresses.len());
                for addr_match in &found_addresses {
                    if let Some(balance) = addr_match.balance {
                        log_msg.push_str(&format!("\n[{}]   → {} (Balance: {} BTC)", 
                            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                            addr_match.address, balance));
                    }
                }
                let _ = log_mgr.write_log(&log_msg);
            }

            // Generate detailed reports
            for &idx in &matched_results {
                let result = &results[idx];
                let result_found_addrs: Vec<AddressMatch> = found_addresses
                    .iter()
                    .filter(|addr| {
                        result.addresses.iter().any(|a| a.address == addr.address)
                    })
                    .cloned()
                    .collect();

                generate_report(result, &result_found_addrs, &args.output_dir)?;
            }

            stats.print_stats(args.simple);
            
            // Send email notification if configured
            if let Some(ref email) = args.email {
                send_email_notification(email, &found_addresses)?;
            }
            
            return Ok(());
        }

        batch_count += 1;
        stats.batches_processed.fetch_add(1, Ordering::Relaxed);
    }

    stats.print_stats(args.simple);
    
    if args.simple {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        eprintln!("[{}] Completed {} batches. No matches found.", timestamp, batch_count);
    } else {
        eprintln!("\nCompleted {} batches. No matches found.", batch_count);
    }

    Ok(())
}

