use crate::bossman::Job;
use redis::AsyncCommands;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unable to establish a connection to redis: {0}")]
    UnableToEstablishConnection(redis::RedisError),

    #[error("encoding to binary failed")]
    EncodingFailed,

    #[error("decoding from binary failed")]
    DecodingFailed,

    #[error("unable to save: {0}")]
    UnableToSave(redis::RedisError),

    #[error("unable to read: {0}")]
    UnableToRead(redis::RedisError),
}

pub async fn save_job(job: &Job) -> Result<(), Error> {
    let mut conn = connect().await?;

    let encoded_job: Vec<u8> = bincode::serialize(&job).map_err(|_| Error::EncodingFailed)?;

    let _: () = conn
        .set(&job.id, encoded_job)
        .await
        .map_err(Error::UnableToSave)?;

    Ok(())
}

pub async fn get_job(id: &str) -> Result<Job, Error> {
    let mut conn = connect().await?;

    let encoded_job: Vec<u8> = conn.get(id).await.map_err(Error::UnableToRead)?;

    bincode::deserialize(&encoded_job).map_err(|_| Error::DecodingFailed)
}

async fn connect() -> Result<redis::aio::Connection, Error> {
    let client =
        redis::Client::open("redis://127.0.0.1/").map_err(Error::UnableToEstablishConnection)?;

    client
        .get_async_connection()
        .await
        .map_err(Error::UnableToEstablishConnection)
}
