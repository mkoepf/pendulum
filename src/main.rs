mod solver;

use solver::Solver;

/// Solve
/// x'' = a*sin(x)
///
/// or, equivalently
///
/// x' = y
/// y' = a*sin(x)

fn main() {

    match Solver::new() {
        Ok(s) =>
            match s.solve() {
                Ok(solution) => println!("Solution: {}", solution.stats),
                Err(e) => println!("Integration failed: {}", e.to_string()),
            }
        Err(e) => println!("Initialization failed: {}", e.to_string()),
    }
}
