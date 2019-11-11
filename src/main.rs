use ode_solvers::*;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::error::Error;

type Time = f64;
type State = Vector2<f64>;

struct Parameters {
    a: f64,
}

impl ode_solvers::System<State> for Parameters {
    fn system(&self, _t: Time, y: &State, dy: &mut State)  {
        dy[0] = y[1];
        dy[1] = self.a *  y[0].sin();
    }
}

/// Solve
/// x'' = a*sin(x)
///
/// or, equivalently
///
/// x' = y
/// y' = a*sin(x)
fn main() {

    let system = Parameters {a: -1.0};

    let y0 = State::new(1.0, 0.0);

    let mut stepper = Dopri5::new(system, 0.0, 8.0*std::f64::consts::PI, 1.0e-2, y0, 1.0e-10, 1.0e-10);
    let res = stepper.integrate();

    match res {
       Ok(stats) => {

            stats.print();

            let x_out = stepper.x_out();
            let y_out = stepper.y_out();

            let path = Path::new("out.csv");

            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", path.display(), why.description()),
                Ok(file) => file,
            };

            let solution = x_out.iter().zip(y_out);

            let solution_strings: Vec<String> = solution.map(|(x,y)| format!("{},{},{}", x, y[0], y[1])).collect();

            match writeln!(file, "{}", solution_strings.join("\n")) {
                Err(why) => panic!("couldn't write to {}: {}", path.display(), why.description()),
                Ok(_) => println!("successfully wrote to {}", path.display())
            }

        },
        Err(_) => println!("An error occured!")
    }

}
