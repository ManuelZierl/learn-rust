// TASK 3 Roots of Polynomials

pub enum Polynomial<Cof,Exp>
{
    Nil,
    Cons(Cof, Exp, Box<Polynomial<Cof,Exp>>)
}
impl Polynomial<i32, i32>
{
    pub fn new() -> Polynomial<i32,i32> { Polynomial::Nil }
    pub fn print(&self)
    {
        match &self
        {
            Polynomial::Nil => {println!(" 0 "); },
            Polynomial::Cons(c, e, l) => {print!("{}x^{} + ", c, e); l.print(); }
        }
    }
    pub fn add(self, cof: i32, exp: i32) -> Polynomial<i32, i32>
    {
        Polynomial::Cons(cof, exp, Box::new(self))
    }


    pub fn evaluate(&self, x:f32) -> f32
    {
        match &self
        {
            Polynomial::Nil => {0.0},
            Polynomial::Cons(c,e,l) => {(*c as f32)*x.powi(*e) + l.evaluate(x)}
        }
    }

    pub fn derivative(&self) -> Polynomial<i32, i32>
    {
        match &self
        {
            Polynomial::Nil => { Polynomial::Nil}
            Polynomial::Cons(_,0,l) => {l.derivative()}
            Polynomial::Cons(c,e,l) =>
            {
                l.derivative().add(c*e,e-1)
            }

        }
    }

    fn newton_help(&self, x:f32) -> f32
    {
        let f_x = &self.evaluate(x);
        let f_dx =&self.derivative().evaluate(x);
        x - (f_x/f_dx)
    }

    pub fn newton(&self, mut x:f32) -> f32
    {
        let mut prec = 10.;
        while prec > 0.1
        {
            let nx = self.newton_help(x);
            prec = (x - nx).abs();
            x = nx;
        }
        x
    }

}
