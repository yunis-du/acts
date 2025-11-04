# acts-postgres

The acts postgres plugin for acts. 

## Installation

create `config/acts.toml` in current dir
```no_compile
[postgres]
database_url = "postgresql://<your connection string>"
```

```bash,no_compile
cargo add acts-store-postgres --git https://github.com/yunis-du/acts.git
```

## Example

```rust,no_run
use acts::{EngineBuilder, Result};
use acts_store_postgres::PostgresStore;

#[tokio::main]
async fn main() -> Result<()> {
    let engine = EngineBuilder::new()
        .add_plugin(&PostgresStore)
        .build().await?
        .start();

    Ok(())
}
```