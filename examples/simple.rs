use symr::core::expr::{Expr, Constant};

fn main() {
    let x = Expr::Symbol("x".into());
    let y = Expr::Symbol("x".into());
    let z = &(&x + &Expr::Constant(Constant::Integer(2))) * &y;
    println!("({} + 2) * {} = {}", x, y, z);
}
