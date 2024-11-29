use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

///// Represents different log levels for classification
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

/// Structured representation of a log entry
#[derive(Debug)]
struct LogEntry {
    timestamp: String,
    level: LogLevel,
    message: String,
    service: String,
}

/// Log parser and analyzer
struct LogAnalyzer {
    log_entries: Vec<LogEntry>,
}

impl LogAnalyzer {
/// Create a new LogAnalyzer by parsing logs from a file
    fn new(filepath: &str) -> Result<Self, std::io::Error> {
        let file = File::open(Path::new(filepath))?;
        let reader = BufReader::new(file);

        let log_entries: Vec<LogEntry> = reader
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| Self::parse_log_line(&line))
            .collect();

        Ok(LogAnalyzer { log_entries })
    }

    /// Parse a single log line into a LogEntry
    fn parse_log_line(line: &str) -> Option<LogEntry> {
// Expected log format: "TIMESTAMP|LEVEL|SERVICE|MESSAGE"
        let parts: Vec<&str> = line.split('|').collect();
        
        if parts.len() != 4 {
            return None;
        }

        let level = match parts[1] {
            "INFO" => LogLevel::Info,
            "WARNING" => LogLevel::Warning,
            "ERROR" => LogLevel::Error,
            "CRITICAL" => LogLevel::Critical,
            _ => return None,
        };

        Some(LogEntry {
            timestamp: parts[0].to_string(),
            level,
            service: parts[2].to_string(),
            message: parts[3].to_string(),
        })
    }

     /// Generate a comprehensive log report from the parsed log entries
    fn generate_report(&self) -> LogReport {
        let total_entries = self.log_entries.len();
        
        let level_counts: HashMap<LogLevel, usize> = self.log_entries
            .iter()
            .fold(HashMap::new(), |mut acc, entry| {
                *acc.entry(entry.level.clone()).or_insert(0) += 1;
                acc
            });

        let service_counts: HashMap<String, usize> = self.log_entries
            .iter()
            .fold(HashMap::new(), |mut acc, entry| {
                *acc.entry(entry.service.clone()).or_insert(0) += 1;
                acc
            });

        LogReport {
            total_entries,
            level_counts,
            service_counts: service_counts.clone(), // Clone the service_counts
            unique_services: service_counts.keys().cloned().collect(),
        }
    }
}

/// Structured report of log analysis
struct LogReport {
    total_entries: usize,
    level_counts: HashMap<LogLevel, usize>,
    service_counts: HashMap<String, usize>,
    unique_services: HashSet<String>,
}

impl LogReport {
/// Print a detailed, human-readable report of the log analysis
    fn print(&self) {
        println!("Log Analysis Report");
        println!("==================");
        println!("Total Log Entries: {}", self.total_entries);
        
        println!("\nLog Level Distribution:");
        for (level, count) in &self.level_counts {
            println!("  {:?}: {} entries", level, count);
        }

        println!("\nService Activity:");
        for (service, count) in &self.service_counts {
            println!("  {}: {} log entries", service, count);
        }

        println!("\nUnique Services:");
        for service in &self.unique_services {
            println!("  - {}", service);
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let analyzer = LogAnalyzer::new(r"C:\Users\VICKTHUR\Desktop\New folder\system.log")?;
    let report = analyzer.generate_report();
    report.print();
    
    Ok(())
}