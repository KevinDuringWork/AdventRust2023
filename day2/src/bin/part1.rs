// https://siedentop.dev/posts/rust-z3/

use std::fs::read_to_string;
use z3::*;

fn check_line(input:&str) -> i64 {

    // Set up Z3 context.
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);

    // declare variables in Z3 (integers)
    let red = ast::Int::new_const(&ctx, "red");
    let green = ast::Int::new_const(&ctx, "green");
    let blue = ast::Int::new_const(&ctx, "blue");

    // known constraints
    opt.assert(&red.le(&ast::Int::from_u64(&ctx, 12)));
    opt.assert(&green.le(&ast::Int::from_u64(&ctx, 13)));
    opt.assert(&blue.le(&ast::Int::from_u64(&ctx, 14)));

    // transform into Z3 assertions
    let parts: Vec<&str> = input.split(":").collect();
    let game: Vec<&str> = parts[0].split(" ").collect();
    let sets: Vec<&str> = parts[1].trim().split(";").map(|x| x.trim()).collect();
    for set in sets {
        let assertions: Vec<&str> = set.split(", ").collect();
        for assertion in assertions {
            let assertion_parts: Vec<&str> = assertion.split(" ").collect();
            let value = (assertion_parts[0]).parse::<u64>().unwrap();
            let color = assertion_parts[1];

            match color {
                "red" => opt.assert(&red.ge(&ast::Int::from_u64(&ctx, value))),
                "green" => opt.assert(&green.ge(&ast::Int::from_u64(&ctx, value))),
                "blue" => opt.assert(&blue.ge(&ast::Int::from_u64(&ctx, value))),
                _ => ()
            }
        }
    }

    let result:SatResult = opt.check(&[]);

    match result {
        SatResult::Sat => game[1].parse::<i64>().unwrap(),
        SatResult::Unsat => 0,
        SatResult::Unknown => panic!("Cannot be solved")
    }
}

fn main() {
    let lines :Vec<_> = read_to_string("test1.txt")
        .unwrap().lines().into_iter()
        .map(|x| check_line(x))
        .collect();

    dbg!(lines.iter().sum::<i64>());
}
