use tokio::sync::mpsc;
use tokio::sync::oneshot;

use std::future::Future;

mod query_output;
pub use query_output::*;

/// A helper type for database queries. It's a closure that takes a "T" (your connection), and
/// returns a future with output "U" (the result from the database query using the connection.)
pub type DbQuery<T, U> =
    Box<dyn FnOnce(&mut T) -> Box<dyn Future<Output = U> + Unpin + Send> + Send>;

/// A type used to communicate with a database connection across multiple threads using
/// message passing
/// Clone should be used to make a new instance of the database after the first, much like Arc.
#[derive(Clone)]
pub struct Db<Conn: sqlx::Connection + 'static, Output> {
    // A channel used to send information to the database.
    // the data from DbQuery will be sent back from the sent channel.
    writer: mpsc::Sender<(DbQuery<Conn, Output>, oneshot::Sender<Output>)>,
}

#[allow(dead_code)]
impl<Conn, Output> Db<Conn, Output>
where
    Conn: sqlx::Connection + 'static,
    Output: Send + 'static,
{
    pub fn new(mut db: Conn) -> Self {
        let (writer, mut reader): (
            mpsc::Sender<(DbQuery<Conn, Output>, oneshot::Sender<Output>)>,
            _,
        ) = mpsc::channel(50);
        tokio::spawn(async move {
            while let Some((query, return_channel)) = reader.recv().await {
                return_channel
                    .send(query(&mut db).await)
                    .unwrap_or_else(|_| unreachable!());
            }
        });
        Self { writer }
    }

    /// Execute a database query and return the result.
    pub async fn query(&mut self, q: DbQuery<Conn, Output>) -> Output {
        let (tx, rx) = oneshot::channel();
        self.writer
            .send((q, tx))
            .await
            .unwrap_or_else(|_| unreachable!());
        rx.await.expect(
            "This exception should never run as the return_channel shouldn't be dropped \
            until the output is sent over the channel.",
        )
    }
}
