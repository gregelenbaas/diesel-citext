# diesel-citext &emsp; 

Diesel support for Postgres citext Extension

### Example usage:

This very small extension allows you to use the citext type with diesel, without having to make 
modifications to the schema.rs generated file. It takes much inspiration from [diesel-postgis](https://github.com/Boscop/diesel-geography). 

Before sending to postgres and after parsing from postgres, the text content is always passed 
through the Rust String to_lowercase method. 

To Ensure citext in the schema.rs file is parsed correctly, add diesel_citext to the import types 
declaration in your  diesel.toml file.

E.g. it will look like this:
```toml
[print_schema]
file = "src/schema.rs"

import_types = ["diesel::sql_types::*", "diesel_citext::sql_types::*"]
```
