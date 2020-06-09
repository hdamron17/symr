use itertools::Itertools;

#[derive(Clone)]
pub enum Expr {
    Symbol(String),
    Constant(Constant),
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
}

#[derive(Copy, Clone)]
pub enum Constant {
    Integer(i64), // TODO use arbitrary precision
    Rational(i64, i64),
    Real(f64),
}

impl<'a> std::ops::Add<&'a Expr> for &'a Expr {
    type Output = Expr;
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (&Expr::Add(ref xs), &Expr::Add(ref ys)) => {
                let mut xs = xs.clone();
                xs.append(&mut ys.clone());
                Expr::Add(xs)
            }
            (&Expr::Add(ref xs), y) => {
                let mut xs = xs.clone();
                xs.push(y.clone());
                Expr::Add(xs)
            }
            (x, &Expr::Add(ref ys)) => {
                let mut ys = ys.clone();
                ys.insert(0, x.clone());
                Expr::Add(ys)
            }
            (x, y) => Expr::Add(vec![x.clone(), y.clone()]),
        }
    }
}

impl<'a> std::ops::Mul<&'a Expr> for &'a Expr {
    type Output = Expr;
    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (&Expr::Mul(ref xs), &Expr::Mul(ref ys)) => {
                let mut xs = xs.clone();
                xs.append(&mut ys.clone());
                Expr::Mul(xs)
            }
            (&Expr::Mul(ref xs), y) => {
                let mut xs = xs.clone();
                xs.push(y.clone());
                Expr::Mul(xs)
            }
            (x, &Expr::Mul(ref ys)) => {
                let mut ys = ys.clone();
                ys.insert(0, x.clone());
                Expr::Mul(ys)
            }
            (x, y) => Expr::Mul(vec![x.clone(), y.clone()]),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Symbol(s) => write!(f, "{}", s),
            Self::Constant(c) => write!(f, "{}", c),
            Self::Add(xs) => write!(f, "({})", xs.iter().join(" + ")),
            Self::Mul(xs) => write!(f, "({})", xs.iter().join(" * ")),
        }
    }
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Integer(x) => write!(f, "{}", x),
            Self::Rational(x, y) => write!(f, "({}/{})", x, y),
            Self::Real(x) => write!(f, "{}", x),
        }
    }
}
