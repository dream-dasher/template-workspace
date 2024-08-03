
[Cargo Workspace - Ch 12](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)


## Start a Workspace
1. Create dir.
2. Manually add `Cargo.toml` with just a `[workspace]` section & `resolver = 2`.
3. Use `cargo new #### --bin/lib` to add packages (~'sub-repos', technically supert set of 'crate's).

## Crate Inter-Operation
- Add local crate dependency by path: e.g. `other_crate = { path = "../other_crate" }`

## Testing
- Tests will all run together by default.
- `cargo (nex)test (run) --package/-p <package>` will run just that kiddo
