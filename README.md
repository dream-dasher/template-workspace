
[Cargo Workspace - Ch 12](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)


## Start a Workspace
1. Create dir.
2. Manually add `Cargo.toml` with just a `[workspace]` section & `resolver = 2`.
3. Use `cargo new #### --bin/lib` to add packages (~'sub-repos', technically superset of 'crate's).

## Crate Inter-Operation
- Add local crate dependency by path: e.g. `other_crate = { path = "../other_crate" }`

## Testing
- Tests will all run together by default.
- `cargo (nex)test (run) --package/-p <package>` will run just that package

## Gotchas
There is **NO** `[workspace.dev-dependencies]`. *Just* `[workspace.dependencies]`.
The `dev-` element exists in sub-packages, but not workspace root.
(The errors for this are not helpful.)

## Common Dependencies
### Common
```toml
derive_more = "0.99"
itertools = "0.13"
chrono = "0.4"
regex = { version = "1", features = ["logging"] }
csv = "1"
include_dir = "0.7"
walkdir = "2"
```

### Logging
```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["chrono", "env-filter"] }
# tracing-error = "0.2.0"
```

### Async
```toml
futures = "0.3"
tokio = { version = "1", features = ["full", "tracing"] }
sqlx = ...
reqwest = ...
```

### CLI
```toml
clap = { version = "4.5", features = ["env", "derive", "string", "wrap_help"] }
owo-colors = "4"
indicatif = "0.17"
dialoguer = "0.11"
```

### (E)egui
```toml
eframe = "0.27"
egui = "0.27"
egui_extras = "0.27"
egui_inbox = "0.4"
```

### Testing
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
test-log = { version = "0.2", features = ["trace"] }
# tempfile = "3"
# wiremock = "0.5"
```
