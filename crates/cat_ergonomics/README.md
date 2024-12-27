# Package **cat-ergonomics** Notes (README)

### Derive_More

#### Error
- `source` should only be used for types implementing `core::error::Error`, rename otherwise
- use along with `From` & `Display`
  - `#[from(ignore)]` for conflicting auto-implementations
```rust
use derive_more::{Display, Error, From};

#[derive(Debug, Display, From, Error)]
pub enum ErrKind {
        //
        // `custom` errors
        #[from(ignore)] // manually generate; would conflict with `OtherStringError` auto-derive
        #[display("Error extracting lines from input: {}", source_input)]
        NoInputLines { source_input: String },
        //
        // `package` errors
        #[display("CLI parsing library error: {}", source)]
        Clap { source: clap::Error },
        #[display("io error: {}", source)]
        Io { source: io::Error },
        #[display("Error setting tracing subscriber default: {}", source)]
        TracingSubscriber { source: SetGlobalDefaultError },
        //
        // `other` errors
        #[from(ignore)] // use `make_dyn_error` instead; would conflict with auto-derives
        #[display("Uncategorized Error (dyn error object): {}", source)]
        OtherDynError {
                source: Box<dyn std::error::Error + Send + Sync>,
        },
        #[display(r#"Uncategorized string err: "{}""#, source_string)]
        OtherStringError { source_string: String },
        //
        // // common error types for quick access
        // ... 
```

### Crates

### Key Ideas

### Gotchas

### Syntax References

### Needs
