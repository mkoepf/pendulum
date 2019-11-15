use serde::Deserialize;
use config::*;
use ode_solvers::dop_shared::{Stats, IntegrationError};
use ode_solvers::{Dopri5, Vector2};

type Time = f64;
type State = Vector2<f64>;

#[derive(Debug, Deserialize)]
struct PhysicalParameters {
    g: f64,
}

#[derive(Debug, Deserialize)]
struct SolverParameters {
    x_end: f64,
}

#[derive(Debug, Deserialize)]
pub struct Solver {
    physical_parameters: PhysicalParameters,
    solver_parameters: SolverParameters,
}

impl ode_solvers::System<State> for Solver {
    fn system(&self, _t: Time, y: &State, dy: &mut State)  {
        dy[0] = y[1];
        dy[1] = - self.physical_parameters.g * y[0].sin();
    }
}

impl Solver {
    pub fn new() -> Result<Self, ConfigError> {

        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config.toml"))?;

        // Add in settings from the environment
        s.merge(Environment::new())?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }

    pub fn solve(self) -> Result<Stats, IntegrationError> {
        let y0 = State::new(1.0, 0.0);
        let mut stepper = Dopri5::new(self, 0.0, 8.0*std::f64::consts::PI, 1.0e-2, y0, 1.0e-10, 1.0e-10);

        stepper.integrate()

//        let z = stepper.x_out().iter().zip(stepper.y_out());

    }

}