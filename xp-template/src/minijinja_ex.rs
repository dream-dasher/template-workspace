//! minijinja examples

use crate::error::Result;
pub fn mini_jinja_example() -> Result<()> {
        use minijinja::{Environment, context};
        let mut env = Environment::new();
        env.add_template("hello_template", "Hello {{ name }}!")
                .unwrap();
        let tmpl = env.get_template("hello_template").unwrap();
        println!("{}", tmpl.render(context!(name => "John")).unwrap());
        Ok(())
}
