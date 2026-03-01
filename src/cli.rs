use clap::{Parser, ValueEnum};
use std::net::SocketAddr;

/*
 *
 * Example runs:
 * cargo run <domainName>
 * cargo run -- <domainName> --type MX
 * cargo run -- <domainName> --type aaaa --mode recursive --verbose
 *
 * */
#[derive(Parser, Debug)]
#[command(
    name = "dns_resolver",
    version = "0.1.0",
    about = "A toy dns resolver using Hickory DNS for learning",
    long_about = None,
    )]
pub struct Args {
    pub domain: String,

    // DNS record type for querying
    #[arg(short = 't', long = "type", value_enum, default_value = "a")]
    pub record_type: CliRecordType,

    // Resolution mode: stub forwards to the upstream resolver
    // Recursive mode: iterates from teh DNS root servers
    #[arg(short = 'm', long = "mode", value_enum, default_value = "stub")]
    pub mode: Mode,

    //NOTE: upstream resolver address will be used only in stub mode
    #[arg(short = 'u', long = "upstream", default_value = "1.1.1.1:53")]
    pub upstream: SocketAddr,

    // output raw json
    #[arg(long = "json", default_value = "false")]
    pub json: bool,

    // disables in memory cache by default
    #[arg(long = "no-cache", default_value = "false")]
    pub no_cache: bool,

    #[arg(short = 'v', long = "verbose", default_value = "false")]
    pub verbose: bool,

    #[arg(long = "max-hops", default_value = "16")]
    pub max_hops: u8,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq, Hash)]
pub enum CliRecordType {
    // IPv4 address record
    A,
    // IPv6 address record
    Aaaa,
    // Canonical name or alias record
    Cname,
    // Mail exchange record
    Mx,
    // Name server record
    Ns,
    // Text record
    Txt,
    // Start of authority record
    Soa,
}

impl std::fmt::Display for CliRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliRecordType::A => write!(f, "A"),
            CliRecordType::Aaaa => write!(f, "AAAA"),
            CliRecordType::Cname => write!(f, "CNAME"),
            CliRecordType::Mx => write!(f, "MX"),
            CliRecordType::Ns => write!(f, "NS"),
            CliRecordType::Txt => write!(f, "TXT"),
            CliRecordType::Soa => write!(f, "SOA"),
        }
    }
}

// Resolution Mode
#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Mode {
    Stub,
    Recursive,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Stub => write!(f, "stub"),
            Mode::Recursive => write!(f, "recursive"),
        }
    }
}
