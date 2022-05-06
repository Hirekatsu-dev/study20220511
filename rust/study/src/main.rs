extern crate tokio_postgres;

#[macro_use]
extern crate string_enum;

use tokio_postgres::NoTls;
use postgres_types::FromSql;
use std::str::{FromStr, from_utf8};
use thiserror::Error;

#[derive(StringEnum)]
enum SampleKbn {
    /// `00101`
    Hoge,
    /// `00102`
    Fuga,
}

#[derive(Debug, Error)]
enum KbnError {
    #[error("ParseError: {0}")]
    ParseError(String)
}


impl FromSql<'_> for SampleKbn {
    fn from_sql(ty: &postgres_types::Type, raw: &[u8])-> Result<SampleKbn, std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>> {

        println!("from_sql({:?}, {:?})", ty, raw);

        let s = from_utf8(raw)?;

        match SampleKbn::from_str(s) {
            Ok(kbn) => { Ok(kbn) },
            Err(_) => { Err(Box::new(KbnError::ParseError(format!("unknown string: {}", s)))) }
        }
    }

    postgres_types::accepts!(TEXT);
}


#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
    let pg_params = format!("host={} user={} password={}", "db", "user", "pass");
    let (client, connection) = tokio_postgres::connect(&pg_params, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows: Vec<SampleKbn> = client.query("SELECT $1", &[])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect()
    ;

    println!("{:?}", rows);

    Ok(())
}