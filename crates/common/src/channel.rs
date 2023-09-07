use clap::ValueEnum;
use schematic::{derive_enum, ConfigEnum};

derive_enum!(
    #[derive(ConfigEnum, Default, ValueEnum)]
    pub enum Channel {
        #[default]
        Stable, // latest
        Unstable,     // next, beta, alpha, etc
        Experimental, // one-off
        Nightly,
    }
);
