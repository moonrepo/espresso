use schematic::{derive_enum, Config, ConfigEnum};
use std::collections::BTreeMap;

#[derive(Config)]
#[config(allow_unknown_fields, rename_all = "camelCase")]
pub struct CompilerOptions {
    pub allow_arbitrary_extensions: Option<bool>,

    pub allow_importing_ts_extensions: Option<bool>,

    pub allow_js: Option<bool>,

    pub allow_synthetic_default_imports: Option<bool>,

    pub allow_umd_global_access: Option<bool>,

    pub allow_unreachable_code: Option<bool>,

    pub allow_unused_labels: Option<bool>,

    pub always_strict: Option<bool>,

    pub assume_changes_only_affect_direct_dependencies: Option<bool>,

    pub base_url: Option<String>,

    pub check_js: Option<bool>,

    pub composite: Option<bool>,

    pub custom_conditions: Option<Vec<String>>,

    pub declaration: Option<bool>,

    pub declaration_dir: Option<String>,

    pub declaration_map: Option<bool>,

    pub diagnostics: Option<bool>,

    pub disable_referenced_project_load: Option<bool>,

    pub disable_size_limit: Option<bool>,

    pub disable_solution_searching: Option<bool>,

    pub disable_source_of_project_reference_redirect: Option<bool>,

    pub downlevel_iteration: Option<bool>,

    #[setting(rename = "emitBOM")]
    pub emit_bom: Option<bool>,

    pub emit_declaration_only: Option<bool>,

    pub emit_decorator_metadata: Option<bool>,

    pub es_module_interop: Option<bool>,

    pub exact_optional_property_types: Option<bool>,

    pub experimental_decorators: Option<bool>,

    pub explain_files: Option<bool>,

    pub extended_diagnostics: Option<bool>,

    pub force_consistent_casing_in_file_names: Option<bool>,

    pub generate_cpu_profile: Option<bool>,

    pub import_helpers: Option<bool>,

    pub incremental: Option<bool>,

    pub inline_source_map: Option<bool>,

    pub inline_sources: Option<bool>,

    pub isolated_modules: Option<bool>,

    pub jsx_factory: Option<String>,

    pub jsx_fragment_factory: Option<String>,

    pub jsx_import_source: Option<String>,

    pub jsx: Option<Jsx>,

    pub lib: Option<Vec<String>>,

    pub list_emitted_files: Option<bool>,

    pub list_files: Option<bool>,

    pub map_root: Option<String>,

    pub max_node_module_js_depth: Option<u32>,

    pub module: Option<Module>,

    pub module_detection: Option<ModuleDetection>,

    pub module_resolution: Option<ModuleResolution>,

    pub module_suffixes: Option<Vec<String>>,

    pub new_line: Option<String>,

    pub no_emit_helpers: Option<bool>,

    pub no_emit_on_error: Option<bool>,

    pub no_emit: Option<bool>,

    pub no_error_truncation: Option<bool>,

    pub no_fallthrough_cases_in_switch: Option<bool>,

    pub no_implicit_any: Option<bool>,

    pub no_implicit_override: Option<bool>,

    pub no_implicit_returns: Option<bool>,

    pub no_implicit_this: Option<bool>,

    pub no_lib: Option<bool>,

    pub no_property_access_from_index_signature: Option<bool>,

    pub no_resolve: Option<bool>,

    pub no_unchecked_indexed_access: Option<bool>,

    pub no_unused_locals: Option<bool>,

    pub no_unused_parameters: Option<bool>,

    pub out_dir: Option<String>,

    pub out_file: Option<String>,

    pub paths: Option<BTreeMap<String, Vec<String>>>,

    pub preserve_const_enums: Option<bool>,

    pub preserve_symlinks: Option<bool>,

    pub preserve_watch_output: Option<bool>,

    pub pretty: Option<bool>,

    pub react_namespace: Option<String>,

    pub remove_comments: Option<bool>,

    pub resolve_json_module: Option<bool>,

    pub resolve_package_json_exports: Option<bool>,

    pub resolve_package_json_imports: Option<bool>,

    pub root_dir: Option<String>,

    pub root_dirs: Option<Vec<String>>,

    pub skip_default_lib_check: Option<bool>,

    pub skip_lib_check: Option<bool>,

    pub source_map: Option<bool>,

    pub source_root: Option<String>,

    pub strict_bind_call_apply: Option<bool>,

    pub strict_function_types: Option<bool>,

    pub strict_null_checks: Option<bool>,

    pub strict_property_initialization: Option<bool>,

    pub strict: Option<bool>,

    pub strip_internal: Option<bool>,

    pub target: Option<Target>,

    pub trace_resolution: Option<bool>,

    pub ts_build_info_file: Option<String>,

    pub type_roots: Option<Vec<String>>,

    pub types: Option<Vec<String>>,

    pub use_define_for_class_fields: Option<bool>,

    pub use_unknown_in_catch_variables: Option<bool>,

    pub verbatim_module_syntax: Option<bool>,

    #[setting(nested)]
    pub watch_options: Option<WatchOptions>,

    #[deprecated]
    pub charset: Option<String>,

    #[deprecated]
    pub imports_not_used_as_values: Option<String>,

    #[deprecated]
    pub keyof_strings_only: Option<bool>,

    #[deprecated]
    pub no_implicit_use_strict: Option<bool>,

    #[deprecated]
    pub no_strict_generic_checks: Option<bool>,

    #[deprecated]
    pub out: Option<String>,

    #[deprecated]
    pub preserve_value_imports: Option<bool>,

    #[deprecated]
    pub suppress_excess_property_errors: Option<bool>,

    #[deprecated]
    pub suppress_implicit_any_index_errors: Option<bool>,
}

derive_enum!(
    // https://www.typescriptlang.org/tsconfig#jsx
    #[derive(ConfigEnum)]
    pub enum Jsx {
        React,
        ReactJsx,
        ReactJsxdev,
        ReactNative,
        Preserve,
    }
);

derive_enum!(
    // https://www.typescriptlang.org/tsconfig#module
    #[derive(ConfigEnum)]
    pub enum Module {
        Amd,
        #[variant(value = "commonjs")]
        CommonJs,
        Es6,
        Es2015,
        Es2020,
        Es2022,
        Esnext,
        Node12,
        Node16,
        Nodenext,
        None,
        System,
        Umd,
        #[variant(fallback)]
        Other(String),
    }
);

derive_enum!(
    // https://www.typescriptlang.org/tsconfig#moduleDetection
    #[derive(ConfigEnum)]
    pub enum ModuleDetection {
        Auto,
        Legacy,
        Force,
    }
);

derive_enum!(
    // https://www.typescriptlang.org/tsconfig#moduleResolution
    #[derive(ConfigEnum)]
    pub enum ModuleResolution {
        Bundler,
        Classic,
        Node,
        Node10,
        Node12,
        Node16,
        Nodenext,
    }
);

derive_enum!(
    // https://www.typescriptlang.org/tsconfig#target
    #[derive(ConfigEnum)]
    pub enum Target {
        Es3,
        Es5,
        Es6,
        Es7,
        Es2015,
        Es2016,
        Es2017,
        Es2018,
        Es2019,
        Es2020,
        Es2021,
        Es2022,
        Esnext,
        #[variant(fallback)]
        Other(String),
    }
);

#[derive(Config)]
pub struct WatchOptions {
    pub exclude_directories: Option<Vec<String>>,

    pub exclude_files: Option<Vec<String>>,

    pub fallback_polling: Option<String>,

    pub synchronous_watch_directory: Option<bool>,

    pub watch_directory: Option<String>,

    pub watch_file: Option<String>,
}
