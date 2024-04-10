use deadpool::managed::{Manager, RecycleResult, Metrics};
use sqlx::{PgConnection, Error, Connection};

pub struct ConnectionManager {
    pub connection_string: String
}

impl Manager for ConnectionManager {
    type Type = PgConnection;
    type Error = Error;
    
    async fn create(&self) -> Result<PgConnection, Self::Error> {
        PgConnection::connect(&self.connection_string).await
    }
    
    async fn recycle(&self, _: &mut PgConnection, _: &Metrics) -> RecycleResult<Self::Error> {
        Ok(())
    }
}