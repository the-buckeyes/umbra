use mysql_async::{
    prelude::{FromValue, Queryable},
    Conn, Params, Pool, Row,
};
use std::sync::Arc;

pub mod error;

pub type Repo = Arc<Pool>;

pub type Reply<T> = Result<(Portal, T), error::Error>;

pub struct Tuple(pub Row);

impl Tuple {
    pub fn take<T: FromValue>(&mut self, type_name: &str, index: &str) -> Result<T, FromTupleError> {
        match self.0.take_opt(index) {
            Some(Ok(thing)) => Ok(thing),
            Some(Err(_)) => Err(FromTupleError::FailedToConvertType(self.0.clone(), type_name.to_string(), index.to_string())),
            None => Err(FromTupleError::MissingRequiredAttribute(self.0.clone(), type_name.to_string(), index.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum FromTupleError {
    MisMatchedAttributeCount(Row, String, usize),
    MissingRequiredAttribute(Row, String, String),
    FailedToConvertType(Row, String, String),
}

pub trait TryFromTuple: Sized {
    fn try_from_tuple(tuple: &mut Tuple) -> Result<Self, FromTupleError>;

    fn from_tuple(mut tuple: Tuple) -> Self {
        match Self::try_from_tuple(&mut tuple) {
            Ok(thing) => thing,
            Err(e) => panic!("ERROR :: ROW_PARSE_FAILED :: \n\n{:?}\n", e)
        }
    }
}

pub struct Portal(Conn);

impl Portal {
    pub async fn new(repo: &Repo) -> Result<Portal, error::Error> {
        let conn = repo.get_conn().await?;

        Ok(Portal(conn))
    }

    pub async fn query<P, Q, R>(self, sql: Q,  params:P) -> Reply<Vec<R>>
    where
        P: Into<Params>,
        Q: AsRef<str>,
        R: TryFromTuple
    {
        let result = self.0.prep_exec(sql, params).await?;
        let (conn, rows) = result.map_and_drop(|row| R::from_tuple(Tuple(row))).await?;

        Ok((Self(conn), rows))
    }
}

pub async fn new<S>(database_url: S) -> Result<Repo, error::Error>
where
    S: AsRef<str>,
{
    let pool = Pool::new(database_url.as_ref());

    Ok(Arc::new(pool))
}
