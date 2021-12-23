use anyhow::Result;
use std::path::Path;
use tokio::fs::read_to_string;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct TraceFileOpts {
    // source file path to trace
    src_path: String,

    // list of package prefixes to ignore
    #[builder(default, setter(strip_option))]
    ignores: Option<Vec<String>>,

    // allow static dependencies to be missing
    #[builder(default)]
    bail_on_missing: bool,

    // include source map paths in output
    #[builder(default)]
    include_source_maps: bool,
}

pub async fn trace_file(opts: TraceFileOpts) -> Result<()> {
    let src_path = opts.src_path;
    println!("src_path: {}", src_path);

    // Parameters

    // TODO: Translate this
    // _extraImports = _extraImports || normalizeExtraImports(extraImports);

    // Get source
    let base_dir = Path::new(&src_path)
        .parent()
        .ok_or(anyhow!("Invalid src_path"))?;

    println!("base_dir: {:?}", base_dir);

    let src = read_to_string(src_path).await?;

    println!("File source: {}", src);

    Ok(())
}
