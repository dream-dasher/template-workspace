//! minijinja examples

use crate::error::Result;
pub fn mini_jinja_example() -> Result<()> {
        use minijinja::{context, Environment};
        let mut env = Environment::new();
        env.add_template("hello", "Hello {{ name }}!").unwrap();
        let tmpl = env.get_template("hello").unwrap();
        println!("{}", tmpl.render(context!(name => "John")).unwrap());
        Ok(())
}
