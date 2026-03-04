use serde::{Deserialize, Serialize};

// DNS response packet, returned by either stub or recursive implementation .
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsResponse {
    // domain that was queried
    pub domain: String,

    // record type queried
    pub record_type: String,

    // Records in the answer section
    pub answers: Vec<DnsRecord>,

    // Records in the Authority section
    pub authority: Vec<DnsRecord>,

    // Records in the Additional section
    pub additional: Vec<DnsRecord>,

    pub from_cache: bool,

    pub elapsed_ms: u128,

    // Number of delegation hops taken
    pub hops: u8,

    pub authoritative: bool,

    // upstream resolver used (stub mode)
    // last NS queried (recursive mode)
    pub resolved_via: String,
}

impl DnsResponse {
    pub fn is_empty(&self) -> bool {
        self.answers.is_empty()
    }
}

// A single resource record from any section of a DNS response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    // Owner name
    pub name: String,

    // TTL in the cache
    pub ttl: u32,

    // DNS class
    pub class: String,

    // Record type
    pub kind: String,

    // actual record data
    pub data: RecordData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum RecordData {
    // IPv4 address (A record)
    A(String),

    // IPv6 address (AAAA record)
    Aaaa(String),

    // Canonical name alias (CNAME record)
    Cname(String),

    // Mail exchange (MX record) can have priority
    Mx {
        priority: u16,
        exchange: String,
    },

    // Name server hostname
    Ns(String),

    // Text strings (TXT record)
    Txt(Vec<String>),

    // Start of Authority metadata (SOA record)
    Soa {
        // Primary name server for the zone
        mname: String,
        // Email address of the zone administrator (encoded as DNS name)
        rname: String,
        // Zone serial number
        serial: u32,
        // Seconds between refresh checks
        refresh: u32,
        // Seconds to wait before retrying a failed refresh
        retry: u32,
        // Seconds after which the zone is no longer authoritative
        expire: u32,
        // Minimum TTL for negative caching (RFC 2308)
        minimum: u32,
    },

    // Unknown or unsupported record type
    Unknown(String),
}

impl std::fmt::Display for RecordData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordData::A(ip) => write!(f, "{}", ip),
            RecordData::Aaaa(ip) => write!(f, "{}", ip),
            RecordData::Cname(name) => write!(f, "{}", name),
            RecordData::Ns(name) => write!(f, "{}", name),
            RecordData::Mx { priority, exchange } => write!(f, "{} {}", priority, exchange),
            RecordData::Txt(parts) => write!(f, "\"{}\"", parts.join(" ")),
            RecordData::Soa {
                mname,
                rname,
                serial,
                refresh,
                retry,
                expire,
                minimum,
            } => {
                write!(
                    f,
                    "{} {} {} {} {} {} {}",
                    mname, rname, serial, refresh, retry, expire, minimum
                )
            }
            RecordData::Unknown(raw) => write!(f, "{}", raw),
        }
    }
}
