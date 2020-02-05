use serde::Deserialize;
use config::*;
use ode_solvers::dop_shared::{Stats, IntegrationError};
use ode_solvers::{Dopri5, Vector2};

type Time = f64;
type State = Vector2<f64>;
type SolutionPoint = (Time, State);
type Solution = Vec<SolutionPoint>;

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

fn combine_solution(timeline: &Vec<Time>, states: &Vec<State>) -> Solution {
    let tvec = timeline.to_vec();
    let svec = states.to_vec();

    tvec.iter().cloned()
        .zip(svec)
        .collect::<Solution>()
}

pub struct SolutionData {
    pub solution: Solution,
    pub stats: Stats,
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

    pub fn solve(self) -> Result<SolutionData, IntegrationError> {
        let y0 = State::new(1.0, 0.0);
        let mut stepper = Dopri5::new(self, 0.0, 8.0*std::f64::consts::PI, 1.0e-2, y0, 1.0e-10, 1.0e-10);

        let stats = stepper.integrate()?;

        let solution = combine_solution(stepper.x_out(), stepper.y_out());

        Ok(SolutionData {
            stats: stats,
            solution: solution
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_solution() {
        let s2: State = Vector2::new(0.12, 3.45);
        let s1: State = Vector2::new(4.56, 7.89);

        let t1: Time = 2.34;
        let t2: Time = 5.67;

        let x_out_slice = &vec!(t1, t2);
        let y_out_slice = &vec!(s1, s2);

        let combined_result = combine_solution(x_out_slice, y_out_slice);

        let expectation = vec!((t1, s1), (t2, s2));

        assert_eq!(expectation, combined_result);
    }
}

