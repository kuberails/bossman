use crate::bossman::Job;
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

    #[error("unable to find job for id: {0}")]
    UnableToFindJob(String),

    #[error("unable to find jobs for name: {0}")]
    UnableToFindJobList(String),

    #[error("unmatched redis error: {0}")]
    OtherRedisError(#[from] redis::RedisError),
}

// public methods
pub async fn save_job(job: &Job) -> Result<(), Error> {
    let conn = internal::connect().await?;
    internal::save_job(conn, job).await
}

pub async fn get_job(id: &str) -> Result<Job, Error> {
    let conn = internal::connect().await?;
    internal::get_job(conn, id).await
}

pub async fn get_jobs_by_name(name: &str) -> Result<Vec<Job>, Error> {
    let conn = internal::connect().await?;
    internal::get_jobs_by_name(conn, name).await
}

// implementation private functions
mod internal {
    use crate::bossman::Job;
    use crate::db::Error;
    use redis::aio::Connection;
    use redis::AsyncCommands;

    pub async fn connect() -> Result<Connection, Error> {
        let client = redis::Client::open("redis://127.0.0.1/")
            .map_err(Error::UnableToEstablishConnection)?;

        client
            .get_async_connection()
            .await
            .map_err(Error::UnableToEstablishConnection)
    }

    pub async fn get_job(mut conn: Connection, id: &str) -> Result<Job, Error> {
        let encoded_job: Vec<u8> = conn.get(id).await.map_err(Error::UnableToRead)?;

        match encoded_job.as_slice() {
            [] => Err(Error::UnableToFindJob(id.to_string())),
            _ => bincode::deserialize(&encoded_job).map_err(|_| Error::DecodingFailed),
        }
    }

    pub async fn get_jobs_by_name(mut conn: Connection, name: &str) -> Result<Vec<Job>, Error> {
        let job_ids: Vec<String> = conn.smembers(name).await?;

        match job_ids.as_slice() {
            [] => Err(Error::UnableToFindJobList(name.to_string())),
            [id] => {
                let job = get_job(conn, id).await?;
                Ok(vec![job])
            }
            _ => {
                let encoded_jobs: Vec<Vec<u8>> = conn.get(job_ids).await?;

                Ok(encoded_jobs
                    .into_iter()
                    .filter_map(|encoded_job| bincode::deserialize(&encoded_job).ok())
                    .collect())
            }
        }
    }

    pub async fn save_job(mut conn: Connection, job: &Job) -> Result<(), Error> {
        let encoded_job: Vec<u8> = bincode::serialize(job).map_err(|_| Error::EncodingFailed)?;

        let _: () = conn
            .set(&job.id, encoded_job)
            .await
            .map_err(Error::UnableToSave)?;

        let _: () = conn.sadd(&job.name, &job.id).await?;

        Ok(())
    }
}
