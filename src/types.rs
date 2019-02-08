use std::error::Error;
use std::io::prelude::*;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::pg::Pg;
use crate::sql_types::*;

impl FromSql<Citext, Pg> for String {
	fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		use std::str;
        let string = str::from_utf8(not_none!(bytes))?;
		string.to_lowercase();
        Ok(string.to_owned())
	}
}

impl ToSql<Citext, Pg> for String {
	fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_all(self.to_lowercase().as_bytes())
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error + Send + Sync>)
	}
}

impl ToSql<Citext, Pg> for str {
	fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_all(self.to_lowercase().as_bytes())
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error + Send + Sync>)
	}
}

