//! A SQLite wrapper to use with this rocker project

use std::path::Path;

use sqlite::Connection;

/// Wrapper for (sqlite::open)[sqlite::open]
fn connect<T: AsRef<Path>>(path: T) -> Result<Connection, std::io::Error> {
    if let Ok(conn) = sqlite::open(path) {
        Ok(conn)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Database path not found",
        ))
    }
}
