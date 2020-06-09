use symr::core::expr::{Expr, Constant};

fn main() {
    let x = Expr::Symbol("x".into());
    let y = Expr::Symbol("x".into());
    let z = (x.clone() + Expr::Constant(Constant::Integer(2))) * y.clone();
    println!("({} + 2) * {} = {}", x, y, z);
}
