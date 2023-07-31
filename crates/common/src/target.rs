use clap::ValueEnum;
use schematic::{derive_enum, ConfigEnum};

// We intentionally do not support old targets, like ES5,
// and new targets like ESNext. Only stable targets.
derive_enum!(
    #[derive(ConfigEnum, Copy, Default, ValueEnum)]
    pub enum EsTarget {
        Es2015,
        Es2016,
        Es2017,
        #[default]
        Es2018,
        Es2019,
        Es2020,
        Es2021,
        Es2022,
    }
);
