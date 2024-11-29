//! Serde & Serde_JSON
//! struct W {
//!     a: i32,
//!     b: i32,
//! }
//! let w = W { a: 0, b: 0 }; // Represented as `{"a":0,"b":0}`
//!
//! struct X(i32, i32);
//! let x = X(0, 0); // Represented as `[0,0]`
//!
//! struct Y(i32);
//! let y = Y(0); // Represented as just the inner value `0`
//!
//! struct Z;
//! let z = Z; // Represented as `null`
//!
//! enum E {
//!     W { a: i32, b: i32 },
//!     X(i32, i32),
//!     Y(i32),
//!     Z,
//! }
//! let w = E::W { a: 0, b: 0 }; // Represented as `{"W":{"a":0,"b":0}}`
//! let x = E::X(0, 0);          // Represented as `{"X":[0,0]}`
//! let y = E::Y(0);             // Represented as `{"Y":0}`
//! let z = E::Z;                // Represented as `"Z"`

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, json};
use tracing::Level;

struct ImmA(String);
#[derive(Serialize, Deserialize, Debug)]
struct Person
{
        destro: const_drop::TypeWithDestructor,
        name:   String,
        age:    u8,
        phones: Vec<String>,
}

/// !!! `tag`!!! allows you to add a free type to your struct
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
struct Request
{
        method: String,
        params: Value,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>>
{
        // unsafe {
        //         std::env::set_var("RUST_LOG", "trace");
        // }
        tracing_subscriber::fmt::init();
        println!();
        println!("----------------------------------------");
        tracing::info!("Using a tag.");
        let my_request = Request { method: "say_hello".to_string(), params: json!({"name": "John Doe"}) };
        println!("{}", serde_json::to_string_pretty(&my_request)?);

        println!();
        println!("----------------------------------------");
        let jim = Person {
                destro: const_drop::ZERO_WITH_DESTRUCTOR,
                name:   "Jim".to_string(),
                age:    42,
                phones: vec!["+44 1234567".to_string(), "+44 2345678".to_string()],
        };
        tracing::event!(Level::INFO, ?jim, "Jim");
        println!("Jim: {}", serde_json::to_string_pretty(&jim)?);

        println!();
        println!("----------------------------------------");
        // Note `"0"` instead of `0` for `destro` field will error-out parsing.
        let data_string = r#"
            {
                "destro": 0,
                "name": "John Doe",
                "age": 43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ]
            }"#;
        tracing::event!(Level::INFO, data_string, "data");
        let data_man: Person = serde_json::from_str(data_string)?;
        tracing::event!(Level::INFO, ?data_man, "struct from string");

        println!();
        println!("----------------------------------------");

        Ok(())
}

fn untyped_example() -> Result<()>
{
        // Some JSON input data as a &str. Maybe this comes from the user.
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data)?;

        // Access parts of the data by indexing with square brackets.
        println!("Please call {} at the number {}", v["name"], v["phones"][0]);

        Ok(())
}

/// Re-implementing a module from workspaces `xp-drop` in order to add serde traits.
/// Though seeing how Serde deals with newtype and multiple wrappers will be useful too.
mod const_drop
{
        use serde::{Deserialize, Serialize};
        use tracing::Level;

        pub const ZERO_WITH_DESTRUCTOR: TypeWithDestructor = TypeWithDestructor(0);

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TypeWithDestructor(i32);
        impl Drop for TypeWithDestructor
        {
                fn drop(&mut self)
                {
                        tracing::event!(Level::TRACE, destro_num = self.0);
                }
        }
}
