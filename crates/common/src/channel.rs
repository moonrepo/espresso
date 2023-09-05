use clap::ValueEnum;
use schematic::{derive_enum, ConfigEnum};

derive_enum!(
    #[derive(ConfigEnum, ValueEnum)]
    pub enum Channel {
        Stable,       // latest
        Unstable,     // next, beta, alpha, etc
        Experimental, // one-off
        Nightly,
    }
);
