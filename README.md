# Axum

Example [Axum](https://github.com/tokio-rs/axum) app. Axum is a web framework for Rust from the Tokio project, focused on ergonomics and modularity.

## Deploy settings

| Field | Value |
| --- | --- |
| Build | Railpack |
| Project directory | `/` |
| Install command | leave empty |
| Build command | leave empty |
| Start command | leave empty |
| Application port | `5000` |
| Health check path | `/` |

## <a name="dev"></a>Run locally
```bash
# install dependencies
cargo fetch

# run in dev mode
cargo run

# build for production
cargo run --release
```
