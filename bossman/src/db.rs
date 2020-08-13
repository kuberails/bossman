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

    #[error("redis error: {0}")]
    OtherRedisError(#[from] redis::RedisError),

    #[error("unable to find job for: {0}")]
    NotFound(String),
}

pub async fn save_job(job: &Job) -> Result<(), Error> {
    let mut conn = connect().await?;

    let encoded_job: Vec<u8> = bincode::serialize(&job).map_err(|_| Error::EncodingFailed)?;

    let _: () = conn
        .set(&job.id, encoded_job)
        .await
        .map_err(Error::UnableToSave)?;

    let _: () = conn.sadd(&job.name, &job.id).await?;

    Ok(())
}

pub async fn get_job(id: &str) -> Result<Job, Error> {
    let mut conn = connect().await?;

    let encoded_job: Vec<u8> = conn.get(id).await.map_err(Error::UnableToRead)?;

    deserialize_job(encoded_job, id)
}

pub async fn get_jobs_by_name(name: &str) -> Result<Vec<Job>, Error> {
    let mut conn = connect().await?;

    let job_ids: Vec<String> = conn.smembers(name).await?;


    let encoded_jobs: Vec<Vec<u8>> = multi_get(conn, job_ids, name).await?;

    Ok(encoded_jobs
        .into_iter()
        .filter_map(|encoded_job| bincode::deserialize(&encoded_job).ok())
        .collect())
}

async fn connect() -> Result<redis::aio::Connection, Error> {
    let client =
        redis::Client::open("redis://127.0.0.1/").map_err(Error::UnableToEstablishConnection)?;

    client
        .get_async_connection()
        .await
        .map_err(Error::UnableToEstablishConnection)
}

fn deserialize_job(encoded: Vec<u8>, id: &str) -> Result<Job, Error> {
    if encoded.is_empty() {
        Err(Error::NotFound(id.to_string()))
    } else {
        bincode::deserialize(&encoded).map_err(|_| Error::DecodingFailed)
    }
}

async fn multi_get(
    mut conn: redis::aio::Connection,
    job_ids: Vec<String>,
    name: &str,
) -> Result<Vec<Vec<u8>>, Error> {
    match job_ids.len() {
        0 => Err(Error::NotFound(name.to_string())),
        1 => {
            let id = job_ids
                .first()
                .expect("we already know theres atleast one element");

            let encoded_job: Vec<u8> = conn.get(id).await.map_err(Error::UnableToRead)?;
            Ok(vec![encoded_job])
        }
        _ => Ok(conn.get(job_ids).await?),
    }
}
