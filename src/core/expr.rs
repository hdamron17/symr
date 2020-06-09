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

impl std::ops::Add<Expr> for Expr {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Add(xs), Self::Add(ys)) => {
                let mut xs = xs.clone();
                xs.append(&mut ys.clone());
                Self::Add(xs)
            }
            (Self::Add(xs), y) => {
                let mut xs = xs.clone();
                xs.push(y);
                Self::Add(xs)
            }
            (x, Self::Add(ys)) => {
                let mut ys = ys.clone();
                ys.insert(0, x);
                Self::Add(ys)
            }
            (x, y) => Self::Add(vec![x, y]),
        }
    }
}

impl std::ops::Mul<Expr> for Expr {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Self::Mul(xs), Self::Mul(ys)) => {
                let mut xs = xs.clone();
                xs.append(&mut ys.clone());
                Self::Mul(xs)
            }
            (Self::Mul(xs), y) => {
                let mut xs = xs.clone();
                xs.push(y);
                Self::Mul(xs)
            }
            (x, Self::Mul(ys)) => {
                let mut ys = ys.clone();
                ys.insert(0, x);
                Self::Mul(ys)
            }
            (x, y) => Self::Mul(vec![x, y]),
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
