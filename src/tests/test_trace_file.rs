use anyhow::Result;

use crate::trace_file::{trace_file, TraceFileOpts};

#[tokio::test]
async fn smoke_test() -> Result<()> {
    let opts = TraceFileOpts::builder()
        .src_path("/home/brandon/code".to_string())
        .build();

    trace_file(opts).await?;

    Ok(())
}
