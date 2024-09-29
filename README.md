

## Adjusting this repo
`(-)-p(review)` is on by default, but this will allow renaming a package and references to it.
```bash
cargo clean
rename_files -preview --recurse '^bin_tbd' --rep bin_to_be_dee
sd -preview 'bin_tbd' bintobedet $(find . --type f)
```
## Cargo Generate Notes
[Cargo Generate Documentation](https://cargo-generate.github.io/cargo-generate/templates/builtin_placeholders.html)
### Builtins
- `authors`
- `project_name`/`crate_name`: kebab-case/snake_case
- `crate_type`: bin/lib
- `os-arch`
- `username`
- `within_cargo_project`
- `is_init`
### Syntax
`{{ crate_name }}`, `{{project-name | upper_camel_case}}`

## Workspace Notes
[Cargo Workspace - Ch 12](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

### Start a Workspace
1. Create dir.
2. Manually add `Cargo.toml` with just a `[workspace]` section & `resolver = 2`.
3. Use `cargo new #### --bin/lib` to add packages (~'sub-repos', technically superset of 'crate's).

### Crate Inter-Operation
- Add local crate dependency by path: e.g. `other_crate = { path = "../other_crate" }`

### Testing
- Tests will all run together by default.
- `cargo (nex)test (run) --package/-p <package>` will run just that package

### Gotchas
There is **NO** `[workspace.dev-dependencies]`. *Just* `[workspace.dependencies]`.
The `dev-` element exists in sub-packages, but not workspace root.
(The errors for this are not helpful.)

## External Tools to Add docs and funcs for
- [Cargo-Machete](https://github.com/bnjbvr/cargo-machete)
- [Git-Cliff](https://github.com/orhun/git-cliff)
- [Cargo-PGO](https://github.com/Kobzol/cargo-pgo)
- [Cargo-dist](https://opensource.axo.dev/cargo-dist/book/installers/homebrew.html)

## Common Dependencies
### --Async--
```toml
futures = "0.3"
tokio = { version = "1", features = ["full", "tracing"] }
```

### --CLI--
```toml
clap = { version = "4", features = ["env", "derive", "string", "unicode", "wrap_help"] }
owo-colors = "4"
indicatif = "0.17"
dialoguer = "0.11"
```

### --Diagnostics--
```toml
tracing = "0.1"
# tracing-appender = "0.2"
# tracing-error = "0.2"
tracing-subscriber = { version = "0.3", features = ["chrono", "env-filter"] }
```

### --Env & Files--
```toml
csv = "1"
dotenvy = "0.15"
# include_dir = "0.7"
secrecy = "0.8"
walkdir = "2"
```

### --General--
```toml
bon = "2""
chrono = "0.4"
derive_more = { version = "1", features = [
    "full",
    "constructor",
    "error",
    "deref",
    "from",
] }
itertools = "0.13"
regex = { version = "1", features = ["logging"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### --Networking--
```toml
reqwest = { version = "0.12", features = ["blocking", "json"] }
sqlx = { version = "0.8", features = [
    "chrono",
    "mysql","postgres","sqlite",
    "macros",
    "runtime-tokio",
    "tls-rustls",
] }
```

### --egui--
```toml
eframe = "0.27"
egui = "0.27"
egui_extras = "0.27"
egui_inbox = "0.4"
```

### --Testing--
```toml
quickcheck = "1"
quickcheck_macros = "1"
insta = { version = "1", features = [
        "csv",
        "json",
        "regex",
        "serde",
        "toml",
        "walkdir",
        "yaml",
] 
### --Test Trace-Logging--
test-log = { version = "0.2", features = ["trace"] }

### --Mocking--
# tempfile = "3"
# wiremock = "0.5"
```
