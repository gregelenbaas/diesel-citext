#[derive(QueryId, SqlType)]
#[postgres(type_name = "citext")]
pub struct Citext;
