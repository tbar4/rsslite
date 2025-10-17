// rsslite/utils/src/db_logger.rs
//
// DB logger that implements `std::io::Write` and persists log lines into the
// SQLite database using SeaORM. The writer is deliberately fire‑and‑forget – it
// spawns an async task that performs the insert, so the synchronous `write`
// call never blocks the tracing subscriber.
/* 
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::io::{Error, ErrorKind, Result as IoResult, Write};
use std::sync::Arc;
use tokio::task;

//use models::rss::log;

use tracing_subscriber::fmt::MakeWriter;

/// A writer that stores each line written to it as a row in the `log` table.
///
/// The writer is cheap to clone because it holds an `Arc` to the underlying
/// `DatabaseConnection`. Each call to `write` spawns a background Tokio task,
/// builds an `ActiveModel` for the `log` entity and inserts it. Errors from the
/// insert are intentionally ignored – logging should never cause the
/// application to panic.
#[derive(Clone, Debug)]
pub struct DbWriter {
    db: Arc<DatabaseConnection>,
}

impl DbWriter {
    /// Create a new `DbWriter` from an `Arc<DatabaseConnection>`.
    ///
    /// The `Arc` is stored directly; the caller is responsible for providing
    /// a shared reference to the database connection.
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

// Implement `MakeWriter` so that `DbWriter` can be used with
// `tracing_subscriber::fmt().with_writer(...)`.  Each call clones the writer,
// which is cheap because it only contains an `Arc`.
impl<'a> MakeWriter<'a> for DbWriter {
    type Writer = DbWriter;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

impl Write for DbWriter {
    /// Write a slice of bytes to the logger.
    ///
    /// The bytes are interpreted as UTF‑8. The whole slice is treated as a
    /// single log message (the caller – `tracing_subscriber` – already formats
    /// a line for us). The method returns the number of bytes consumed, or an
    /// I/O error if the input is not valid UTF‑8.
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        // Convert the incoming bytes into a `String`.  If they are not UTF‑8
        // we report an error; this mirrors the behaviour of the standard
        // `std::io::Write` implementations.
        let line = std::str::from_utf8(buf)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?
            .to_string();

        // Clone the `Arc` for the async task.
        let db = self.db.clone();

        // Spawn a fire‑and‑forget task that inserts the log entry.
        // This keeps the `write` method synchronous and fast.
        task::spawn(async move {
            // Populate the `log` active model.  We only store the timestamp
            // and the raw message; other columns (`level`, `target`, `fields`)
            // are left empty for now but can be extended later.
            let am = log::ActiveModel {
                ts: Set(Utc::now().to_rfc3339()),
                level: Set(String::new()),
                target: Set(String::new()),
                message: Set(line),
                fields: Set(None),
                ..Default::default()
            };

            // Perform the insertion; ignore any error because logging should
            // never bring down the application.
            // `db` is an `Arc<DatabaseConnection>`, so we dereference it.
            let _ = am.insert(&*db).await;
        });

        // Report that we consumed the whole buffer.
        Ok(buf.len())
    }

    /// Flush is a no‑op for the DB writer – inserts are already dispatched.
    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}
*/