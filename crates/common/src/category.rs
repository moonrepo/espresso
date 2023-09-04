// Initial list based on Cargo: https://crates.io/category_slugs

use schematic::ConfigEnum;

#[derive(ConfigEnum)]
pub enum Category {
    // General categories
    Accessibility,
    Algorithm,
    Architecture,
    #[variant(alias = "async")]
    Asynchronous,
    Authentication,
    Authorization,
    Caching,
    #[variant(alias = "cli")]
    CommandLineInterface,
    CommandLineUtilities,
    Compiler,
    Compression,
    Concurrency,
    Configuration,
    #[variant(alias = "ci")]
    ContinuousIntegration,
    #[variant(alias = "cd")]
    ContinuousDeployment,
    Cryptography,
    Database,
    DataStructure,
    DateTime,
    Debugging,
    #[variant(alias = "devx")]
    DeveloperExperience,
    DevelopmentTool,
    Encoding,
    FileSystem,
    Gaming,
    Graphics,
    #[variant(alias = "gui")]
    GraphicalInterface,
    InternalTool,
    #[variant(alias = "i18n")]
    Internationalization,
    #[variant(alias = "l10n")]
    Localization,
    Logging,
    Mathematics,
    Media,
    MediaAudio,
    MediaImage,
    MediaVideo,
    Network,
    Os,
    Profiling,
    TemplateEngine,
    Testing,
    TestingUtilities,
    #[variant(alias = "ux")]
    UserExperience,
    #[variant(alias = "ui")]
    UserInterface,
    Virtualization,
    Visualization,
    Wasm,
    WebServer,
    Websocket,
    // Frontend specific
    ApplicationFramework,
    Bundler,
    CodeFormatter,
    CodeGenerator,
    ComponentLibrary,
    Deployment,
    DesignSystem,
    EditorExtension,
    Environment,
    Linter,
    LintRule,
    Plugin,
    Primitives,
    Reactivity,
    Styles,
    Theme,
    TestRunner,
    TypeChecker,
    TypeUtilities,
    ViewFramework,
}
