
use crate::Tokens;
use crate::Operators;

impl Operators {
    fn is_it_operator_type(&self) -> bool {
        use Operators::*;

        match *self {
            Add => return true,
            Subtract => return true,
            Multiply => return true,
            Divide => return true,
            Equal => return true,
            _ => return false,
        }
    }
    fn oper_pirority(&self) -> u32 {
        match *self {
            Operators::Add => 1,
            Operators::Subtract => 1,
            Operators::Multiply => 2,
            Operators::Divide => 2,
            _ => 0,
        }
    }
    fn solve<'a>(&self , a : &Tokens , mut b: Tokens) -> Tokens {
        match *self {
            Operators::Add => {
                let ans = a.get_num() + b.get_num();
                if let Tokens::Num(ref mut num) = b {
                    *num = ans;
                }
                return b;
            },
            Operators::Subtract => {
                let ans = a.get_num() - b.get_num();
                if let Tokens::Num(ref mut num) = b {
                    *num = ans;
                }
                return b;
            },
            Operators::Multiply => {
                let ans = a.get_num() * b.get_num();
                if let Tokens::Num(ref mut num) = b {
                    *num = ans;
                }
                return b;
            }
            Operators::Divide => {
                let ans = a.get_num() / b.get_num();
                if let Tokens::Num(ref mut num) = b {
                    *num = ans;
                }
                return b;
            }
            _ => panic!("any!"),
        }
    }
}

impl Tokens {
    fn is_it_operator_type(&self) -> bool {
        if let Tokens::Oper(_) = self {
            return true;
        }

        false
    }
    fn get_oper_type(&self) -> &Operators {
        if let Tokens::Oper(ref oper) = *self {
            return oper;
        }
        &Operators::None
    }
    fn get_oper_type_in_option(&self) -> Option<&Operators> {
        if let Tokens::Oper(ref oper) = *self {
            return Some(oper);
        }
        None
    }
    fn is_it_token(&self) -> bool {
        match *self {
            Tokens::Str(_) => true,
            Tokens::Num(_) => true,
            Tokens::Bracket{..} => true,
            Tokens::Oper(_) => true,
            Tokens::None => false,
        }
    }
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
    fn token_oper_pirority(&self) -> u32 {
        if let Some(ref oper) = self.get_oper_type_in_option(){
           return oper.oper_pirority();
        }
        0u32
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
    fn get_num(&self) -> f32 {
        if let Tokens::Num(ref num) = *self {
            return *num;
        }
        panic!();
    }
    fn is_it_bracket_token(&self) -> bool {
        if let Tokens::Bracket{..} = *self {
            return true;
        }
        false
    }
}

#[derive(Debug)]
pub struct Values<'a> {
    a: &'a Tokens,
    b: &'a Tokens,
    o: &'a Tokens,
    solved: Tokens,
    need_to_solve: bool,
}

impl<'a> Values<'a> {
    pub fn new() -> Values<'a> {
        Values {
            a: &Tokens::None,
            b: &Tokens::None,
            o: &Tokens::None,
            solved: Tokens::None,
            need_to_solve: false,
        }
    }
    fn does_rule_apply(&self) -> bool {
        if self.o.is_none() && !self.a.is_none() {
            return true;
        }
        if !self.o.is_none() && !self.b.is_none() {
            return true;
        }
        if !self.a.is_none() && !self.o.is_none() && !self.b.is_none() {
            return true;
        }
        false
    }
    fn all_not_none(&self) -> bool {
        if self.a.is_none() {
            return false;
        }
        if self.b.is_none() {
            return false;
        }
        if self.o.is_none() {
            return false;
        }

        true
    }
    fn make_val_none(&mut self) {
        self.a = &Tokens::None;
        self.b = &Tokens::None;
        self.o = &Tokens::None;
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

    // pub fn new() -> TrackSolver<'a> {
    //     TrackSolver {
    //         Val(),
    //         BothSides(),
    //     }
    // }

pub fn solving<'a>(tokenized: &'a Vec<Tokens> , sides: &mut Sides<'a> , values: &mut Values<'a>) {
    let mut present_side;
    for ref token in tokenized {
        if !sides.equal {
            present_side = &mut sides.lhs; 
        } else {
            present_side = &mut sides.rhs;
        }

        if token.is_it_operator_type() {

            if values.all_not_none() {
                if values.o.token_oper_pirority() < token.token_oper_pirority() {
                    present_side.push(values.a);
                    present_side.push(values.o);
                    values.a = values.b;
                    values.b = &Tokens::None;
                    values.o = token;
                } else {
                    let mut a;
                    let mut b;
                    a = &values.a;
                    b = values.b.clone();
                    values.solved = values.o.get_oper_type().solve(&a , b);
                    values.make_val_none();
                }
            } else if values.a.is_none() && values.b.is_not_none() && values.o.is_not_none() && values.b.is_not_none() {
                let mut a;
                let mut b;
                a = &values.solved;
                b = values.b.clone();
                values.solved = values.o.get_oper_type().solve(&a , b);
                values.make_val_none();
            }

            if values.o.is_none() {
                values.o = &token;
            }
        }
        if token.is_it_num() {
            if values.a.is_none() {
                if present_side.len() == 0 && values.solved.is_not_none() {
                    if values.b.is_none() {
                        values.b = &token;
                    }
                } else {
                    values.a = &token;
                }
            } else {
                if values.b.is_none() {
                    values.b = &token;
                }
            }
        }
    }

    if values.all_not_none() {
        let mut a;
        let mut b;
        a = &values.a;
        b = values.b.clone();
        values.solved = values.o.get_oper_type().solve(&a , b);
        values.make_val_none();
    } else if values.a.is_none() && values.b.is_not_none() && values.o.is_not_none() && values.b.is_not_none() {
        let mut a;
        let mut b;
        a = &values.solved;
        b = values.b.clone();
        values.solved = values.o.get_oper_type().solve(&a , b);
        values.make_val_none();
    }
}