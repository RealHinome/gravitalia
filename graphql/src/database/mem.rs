use anyhow::Result;
use r2d2::Pool;
use r2d2_memcache::MemcacheConnectionManager;

#[allow(dead_code)]
/// Represents the value to be stored in Memcached, which can be either a string or a number.
pub enum SetValue {
    /// Stores a value as a string of characters.
    Characters(String),
    /// Stores a value as a 16-bit unsigned number.
    Number(u16),
}

#[derive(Clone, Debug)]
/// Define a structure to manage the Memcached connection pool.
pub struct MemPool {
    pub connection: Pool<MemcacheConnectionManager>,
}

/// Define a trait for the MemcacheManager with methods to interact with Memcached.
pub trait MemcacheManager {
    /// Get data from a given key.
    fn get<T: ToString>(&self, key: T) -> Result<Option<String>>;
    /// Set data in Memcached and return the key.
    fn set<T: ToString>(&self, key: T, value: SetValue) -> Result<String>;
    /// Delete data based on the key.
    fn del<T: ToString>(&self, key: T) -> Result<()>;
}

impl MemcacheManager for MemPool {
    /// Retrieve data from Memcached based on the key.
    fn get<T: ToString>(&self, key: T) -> Result<Option<String>> {
        Ok(self.connection.get()?.get(&key.to_string())?)
    }

    /// Store data in Memcached and return the key.
    fn set<T: ToString>(&self, key: T, value: SetValue) -> Result<String> {
        match value {
            SetValue::Characters(data) => {
                self.connection.get()?.set(&key.to_string(), data, 300)?;
            }
            SetValue::Number(data) => {
                self.connection.get()?.set(&key.to_string(), data, 300)?;
            }
        };

        Ok(key.to_string())
    }

    /// Delete data from Memcached based on the key.
    fn del<T: ToString>(&self, key: T) -> Result<()> {
        self.connection.get()?.delete(&key.to_string())?;

        Ok(())
    }
}

/// Initialize the connection pool for Memcached.
pub fn init(
    config: &crate::model::config::Config,
) -> Result<Pool<MemcacheConnectionManager>> {
    let manager = r2d2_memcache::MemcacheConnectionManager::new(format!(
        "memcache://{}?timeout=2&use_udp=true",
        config.database.memcached.hosts[0]
    ));

    Ok(r2d2_memcache::r2d2::Pool::builder()
        .max_size(
            std::env::var("MEMCACHED_POOL_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse::<u32>()?,
        )
        .min_idle(Some(2))
        .build(manager)?)
}
