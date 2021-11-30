use std::path::Path;

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

pub async fn trace_file(opts: TraceFileOpts) -> Box<Path> {
    let src_path = opts.src_path;

    // Parameters

    // TODO: Translate this
    // _extraImports = _extraImports || normalizeExtraImports(extraImports);

    // Get source
    let src_path = Path::new(&src_path).parent().expect("Invalid src_path");

    Box::new(src_path.to_owned().to_str())
}
