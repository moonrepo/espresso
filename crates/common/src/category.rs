// Initial list based on Cargo: https://crates.io/category_slugs

use schematic::{derive_enum, ConfigEnum};

derive_enum!(
    #[derive(ConfigEnum)]
    pub enum Category {
        // General categories
        #[serde(alias = "a11y")]
        Accessibility,
        Algorithm,
        Architecture,
        #[serde(alias = "async")]
        Asynchronous,
        Authentication,
        Authorization,
        Automation,
        Blockchain,
        Caching,
        #[serde(alias = "cli")]
        CommandLineInterface,
        CommandLineUtilities,
        Compiler,
        Compression,
        Concurrency,
        Configuration,
        #[serde(alias = "ci")]
        ContinuousIntegration,
        #[serde(alias = "cd")]
        ContinuousDeployment,
        #[serde(alias = "cms")]
        ContentManagementSystem,
        Crypto,
        Cryptography,
        Database,
        DataStructure,
        DateTime,
        Debugging,
        #[serde(alias = "devx")]
        DeveloperExperience,
        DevelopmentTool,
        Encoding,
        #[serde(alias = "e2e")]
        EndToEnd,
        ErrorHandling,
        FileSystem,
        Finance,
        Gaming,
        Graphics,
        #[serde(alias = "gui")]
        GraphicalInterface,
        InternalTool,
        #[serde(alias = "i18n")]
        Internationalization,
        #[serde(alias = "l10n")]
        Localization,
        Logging,
        Mathematics,
        MediaProcessing,
        MediaAudio,
        MediaImage,
        MediaVideo,
        Mobile,
        Networking,
        Os,
        Parser,
        Profiling,
        Publishing,
        Rendering,
        Security,
        Social,
        TemplateEngine,
        Testing,
        TestingUtilities,
        TextProcessing,
        #[serde(alias = "ux")]
        UserExperience,
        #[serde(alias = "ui")]
        UserInterface,
        Virtualization,
        Visualization,
        Wasm,
        WebServer,
        Websocket,
        // Frontend specific
        ApplicationFramework,
        Bundler,
        ClientFramework,
        CloudPlatform,
        CodeFormatter,
        CodeGenerator,
        ComponentLibrary,
        CssFramework,
        CssInJs,
        DependencyGraph,
        Deployment,
        DesignSystem,
        EditorExtension,
        Environment,
        Linter,
        LintRules,
        MonorepoTool,
        MonorepoUtilities,
        Plugin,
        Primitives,
        Reactivity,
        ServerFramework,
        #[serde(alias = "ssr")]
        ServerSideRenderer,
        #[serde(alias = "ssg")]
        StaticSiteGenerator,
        Styles,
        StyleGuide,
        TaskRunner,
        Theme,
        TestRunner,
        Toolchain,
        TypeChecker,
        TypeUtilities,
    }
);
