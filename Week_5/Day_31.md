# Rust Log Parser and Analyzer

## Overview

A robust, feature-rich log parsing and analysis tool built in Rust, designed to transform raw log files into meaningful insights about system performance, errors, and service activities.

## Features

- **Comprehensive Log Parsing**
  - Supports structured log entries with timestamp, log level, service, and message
  - Handles multiple log levels (INFO, WARNING, ERROR, CRITICAL)
  - Robust error handling and parsing

- **Advanced Analysis Capabilities**
  - Generate detailed log reports
  - Track log entry timeline
  - Analyze service performance
  - Identify critical messages and error-prone services

- **High-Performance**
  - Leverages Rust's zero-cost abstractions
  - Efficient memory management
  - Minimal runtime overhead


## Key Components

### Log Entry Parsing
- Parse log entries from structured text files
- Support for RFC3339 timestamp format
- Flexible service and message extraction

### Analysis Capabilities
- Log level distribution
- Service activity tracking
- Timeline analysis
- Critical message identification

## Getting Started

### Prerequisites
- Rust (stable version)
- Cargo package manager


## Log Format Specification

### Entry Structure
`TIMESTAMP|LEVEL|SERVICE|MESSAGE`

#### Components
- **Timestamp**: ISO 8601 format (e.g., `2024-01-15T10:30:45Z`)
- **Level**: One of `INFO`, `WARNING`, `ERROR`, `CRITICAL`
- **Service**: Short service name (e.g., `web-service`)
- **Message**: Descriptive event text

## Example Log File

```
2024-01-15T10:30:45Z|INFO|web-service|Server started successfully
2024-01-15T10:31:12Z|WARNING|database-connector|Connection pool reaching maximum capacity
2024-01-15T10:31:45Z|ERROR|authentication-module|Failed login attempt for user 'admin'
```

## Dependencies
- `chrono`: Datetime parsing and manipulation
- Standard Rust libraries

