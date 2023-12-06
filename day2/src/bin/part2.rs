// https://siedentop.dev/posts/rust-z3/

use std::fs::read_to_string;
use std::ptr::write;
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

    // general assertions (can't be negative)
    opt.assert(&red.ge(&ast::Int::from_u64(&ctx, 0)));
    opt.assert(&green.ge(&ast::Int::from_u64(&ctx, 0)));
    opt.assert(&blue.ge(&ast::Int::from_u64(&ctx, 0)));

    // transform into Z3 assertions
    let parts: Vec<&str> = input.split(":").collect();
    let game: Vec<&str> = parts[0].split(" ").collect();
    let sets: Vec<&str> = parts[1].trim().split(";").map(|x| x.trim()).collect();
    for set in sets {
        let assertions: Vec<&str> = set.split(", ").collect();

        //dbg!(&assertions);
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

    opt.minimize(&red);
    opt.minimize(&green);
    opt.minimize(&blue);
    assert_eq!(opt.check(&[]), SatResult::Sat);

    let m = opt.get_model().unwrap();
    let min_red = &m.eval(&red, true).unwrap().as_i64().unwrap();
    let min_green = &m.eval(&green, true).unwrap().as_i64().unwrap();
    let min_blue = &m.eval(&blue, true).unwrap().as_i64().unwrap();
    let result = min_red * min_green * min_blue;

    result
}

fn main() {
    let lines :Vec<_> = read_to_string("test2.txt")
        .unwrap().lines().into_iter()
        .map(|x| check_line(x))
        .collect();

    dbg!(lines.iter().sum::<i64>());
}
