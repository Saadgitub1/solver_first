
use crate::Tokens;
use crate::Operators;

impl Operators {
    fn is_mul(&self) -> bool {
        match *self {
            Operators::Multiply => return true,
            _ => return false,
        }
    }
    fn is_divide(&self) -> bool {
        match *self {
            Operators::Divide => return true,
            _ => return false,
        }
    }
    fn solve(&self , a: &Tokens , b: &Tokens) -> f32 {
        let a = a.get_num();
        let b = b.get_num();
        match *self {
            Operators::Add => return a + b,
            Operators::Subtract => return a - b,
            Operators::Multiply => return a * b,
            Operators::Divide => return a / b,
            _ => panic!(),
        }
    }
    fn mul_oper(&self , a: &Tokens) -> f32 {
        let a = a.get_num();
        match *self {
            Operators::Add => return a,
            Operators::Subtract => return -1f32 * a,
            _ => panic!(),
        }
    }
}

impl Tokens {
    fn is_none(&self) -> bool {
        match *self {
            Tokens::None => return true,
            _ => false,
        }
    }
    fn is_not_none(&self) -> bool {
        match *self {
            Tokens::None => return false,
            _ => true,
        }
    }
    fn is_it_equal_token(&self) -> bool {
        if let Tokens::Oper(Operators::Equal) = *self {
            return true;
        }
        false
    }
    fn is_it_num(&self) -> bool {
        if let Tokens::Num(_) = *self {
            return true;
        }
        false
    }
    fn is_it_oper(&self) -> bool {
        if let Tokens::Oper(_) = *self {
            return true;
        }
        false
    }
    fn get_num(&self) -> f32 {
        if let Tokens::Num(ref num) = *self {
            return *num;
        }
        panic!();
    }
    fn get_oper_type(&self) -> &Operators {
        if let Tokens::Oper(ref oper) = *self {
            return oper;
        }
        &Operators::None
    }
}

#[derive(Debug)]
pub struct Sides<'a> {
    equal: bool,
    pub lhs: Vec<&'a Tokens>,
    pub rhs: Vec<&'a Tokens>,
}

impl<'a> Sides<'a> {
    pub fn new() -> Sides<'a> {
        Sides {
            equal: false,
            lhs: Vec::new(),
            rhs: Vec::new(),
        }
    }
    pub fn make_lhs_rhs(&mut self , tokenized: &'a Vec<Tokens>) {
        for token in tokenized {
            if token.is_it_equal_token() {
                self.equal = true;
                continue;
            }
            if !self.equal {
                self.lhs.push(token);
            } else {
                self.rhs.push(token);
            }
        }
    }
}

pub fn solve_bracket(side: &Vec<Tokens>) -> Tokens {

    let mut inner_vec: Vec<Tokens> = Vec::new();
 
    for token in side {
        if let Tokens::Bracket{bracket: _ , ref content} = *token {
            match inner_vec.last() {
                Some(ref token) => {
                    if let Tokens::Num(_) = *token {
                        inner_vec.push(Tokens::Oper(Operators::Multiply));
                    }
                },
                None => (),
            }
            // if content.len() == 0 {
            //     inner_vec.push(Tokens::Num(1f32));
            // } else {
                inner_vec.push(solve_bracket(content));
            // }
        }
        else {
            let cloned_token = token.clone();
            inner_vec.push(cloned_token);
        }
    }

    //println!("solve bracket {:?}" , inner_vec);
    return solve_divide(&inner_vec);
}

pub fn solve_divide(side: &Vec<Tokens>) -> Tokens {

    let mut inner_vec: Vec<Tokens> = Vec::new();
    let mut solved = Tokens::None;
    let mut a = &Tokens::None;
    let mut b = &Tokens::None;
    let mut o = &Tokens::None;

    for token in side {
        if token.is_it_num() {
            if a.is_none() {
                a = token;
            }
            else if b.is_none() {
                b = token;
                if a.is_not_none() && b.is_not_none() && o.is_not_none() {
                    solved = Tokens::Num(o.get_oper_type().solve(&a , &b));
                    a = &Tokens::None;
                    b = &Tokens::None;
                    o = &Tokens::None;
                } else {
                    inner_vec.push(b.clone());
                }
            }
        }
        else if token.is_it_oper() {
            if token.get_oper_type().is_divide() {
                if o.is_none() {
                    o = token;
                    if solved.is_not_none() && a.is_none() {
                        a = &solved;
                    }
                }
            }
            else {
                if a.is_not_none() {
                    inner_vec.push(a.clone());
                }
                if solved.is_not_none() {
                    inner_vec.push(solved.clone());
                    solved = Tokens::None;
                }
                inner_vec.push(token.clone());
                a = &Tokens::None;
            }
        }
    }

    if a.is_not_none() {
        inner_vec.push(a.clone());
    }
    if solved.is_not_none() {
        inner_vec.push(solved);
    }
    //println!("from divide:{:?}" , inner_vec);
    // println!("{:?} , {:?} , {:?}" , a , b , o);
    return solve_mul(&inner_vec);
}

pub fn solve_mul(side: &Vec<Tokens>) -> Tokens {

    let mut inner_vec: Vec<Tokens> = Vec::new();
    let mut solved = Tokens::None;
    let mut a = &Tokens::None;
    let mut b = &Tokens::None;
    let mut o = &Tokens::None;

    for token in side {
        if token.is_it_num() {
            if a.is_none() {
                a = token;
            }
            else if b.is_none() {
                b = token;
                if a.is_not_none() && b.is_not_none() && o.is_not_none() {
                    solved = Tokens::Num(o.get_oper_type().solve(&a , &b));
                    a = &Tokens::None;
                    b = &Tokens::None;
                    o = &Tokens::None;
                } else {
                    inner_vec.push(b.clone());
                }
            }
        }
        else if token.is_it_oper() {
            if token.get_oper_type().is_mul() {
                if o.is_none() {
                    o = token;
                    if solved.is_not_none() && a.is_none() {
                        a = &solved;
                    }
                }
            }
            else {
                if a.is_not_none() {
                    inner_vec.push(a.clone());
                }
                if solved.is_not_none() {
                    inner_vec.push(solved.clone());
                    solved = Tokens::None;
                }
                inner_vec.push(token.clone());
                a = &Tokens::None;
            }
        }
    }

    if a.is_not_none() {
        inner_vec.push(a.clone());
    }
    if solved.is_not_none() {
        inner_vec.push(solved);
    }
    //println!("from mul:{:?}" , inner_vec);
    // println!("{:?} , {:?} , {:?}" , a , b , o);
    return solve_add_subtract(&inner_vec);
}

pub fn solve_add_subtract(side: &Vec<Tokens>) -> Tokens {

    let mut solved = Tokens::None;
    let mut a = &Tokens::None;
    let mut b = &Tokens::None;
    let mut o = &Tokens::None;

    for token in side {
        if token.is_it_num() {
            if a.is_none() {
                if o.is_not_none() {
                    solved = Tokens::Num(o.get_oper_type().mul_oper(token));
                    a = &solved;
                    o = &Tokens::None;
                } else {
                    a = token;
                }
            }
            else if b.is_none() {
                b = token;
                if a.is_not_none() && b.is_not_none() && o.is_not_none() {
                    solved = Tokens::Num(o.get_oper_type().solve(&a , &b));
                    a = &solved;
                    b = &Tokens::None;
                    o = &Tokens::None;
                }
            }
        }
        else if token.is_it_oper() {
            if o.is_none() {
                o = token;
            }
        }
    }

    if a.is_not_none() {
        solved = a.clone();
    }
    //println!("Ans: {:?}" , solved);
    return solved;
}