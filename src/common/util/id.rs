use snowflake::SnowflakeIdGenerator;
use std::{
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, SystemTime},
};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use ulid::Ulid;
use uuid::Uuid;

static ID: AtomicU64 = AtomicU64::new(1);
pub struct SimpleIdGenerator;
impl SimpleIdGenerator {
    pub fn next() -> u64 {
        ID.fetch_add(1, Ordering::Relaxed)
    }
}

static SNOWFLAKE: OnceLock<Mutex<SnowflakeIdGenerator>> = OnceLock::new();
pub struct SnowflakeGenerator;
impl SnowflakeGenerator {
    pub fn next() -> u64 {
        let base = OffsetDateTime::parse("2025-01-01T00:00:00.000Z", &Rfc3339)
            .unwrap()
            .unix_timestamp();
        let base = SystemTime::UNIX_EPOCH + Duration::from_secs(base as u64);
        let id = SNOWFLAKE
            .get_or_init(|| Mutex::from(SnowflakeIdGenerator::with_epoch(0, 0, base)))
            .lock()
            .unwrap()
            .lazy_generate();
        id as u64
    }
}

pub struct UlidGenerator;
impl UlidGenerator {
    pub fn next() -> String {
        Ulid::new().to_string().to_ascii_lowercase()
    }
}

pub struct UuidGenerator;
impl UuidGenerator {
    pub fn next_v4() -> String {
        Uuid::new_v4().simple().to_string().to_ascii_lowercase()
    }

    pub fn next_v7() -> String {
        Uuid::now_v7().simple().to_string().to_ascii_lowercase()
    }
}
