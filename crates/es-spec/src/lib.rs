use std::fmt::Display;

// We intentionally do not support old targets, like ES5.
#[derive(Clone, Copy, Debug)]
pub enum EsSpec {
    Es2015,
    Es2016,
    Es2017,
    Es2018,
    Es2019,
    Es2020,
    Es2021,
    Es2022,
}

impl Display for EsSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}
