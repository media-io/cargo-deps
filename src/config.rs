/// Configuration options.
///
/// Create this object with `Default::default()` for the configuration equivalent to running without
/// any command-line arguments.
///
/// Please refer to the help menu for information about each option.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct Config {
    pub depth: Option<usize>,
    pub dot_file: Option<String>,
    pub filter: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub include_orphans: bool,
    pub include_versions: bool,
    /// Default: "Cargo.toml".
    pub manifest_path: String,
    pub subgraph: Option<Vec<String>>,
    pub subgraph_name: Option<String>,
    pub registries: Option<Vec<String>>,

    /// Default: true.
    pub regular_deps: bool,
    /// Default: false.
    pub build_deps: bool,
    /// Default: false.
    pub dev_deps: bool,
    /// Default: false.
    pub optional_deps: bool,
    /// Default: true.
    pub transitive_deps: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            depth: None,
            dot_file: None,
            filter: None,
            exclude: None,
            include_orphans: false,
            include_versions: false,
            manifest_path: "Cargo.toml".into(),
            subgraph: None,
            subgraph_name: None,
            registries: None,

            regular_deps: true,
            build_deps: false,
            dev_deps: false,
            optional_deps: false,
            transitive_deps: true,
        }
    }
}
