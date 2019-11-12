use config::{ConfigError, Config, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PhysicalParameters {
    g: f64,
}

#[derive(Debug, Deserialize)]
struct SolverParameters {
   x_end: f64,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    physical_parameters: PhysicalParameters,
    solver_parameters: SolverParameters,
}

// taken from example
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}