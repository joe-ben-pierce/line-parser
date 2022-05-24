use crate::utils::*;



fn is_numeric(input: u8) -> bool{
    is_digit(input) | (input == '.' as u8)
}

fn is_number(s: &str) -> bool{
    match s.parse::<f64>(){
        Ok(_) => true,
        Err(_) => false
    }
}

fn precedence(op: &str) -> Result<u8, String>{
    match op{
        "+" | "-" => Ok(1),
        "*" | "/" => Ok(2),
        "^" => Ok(3),
        _ => Err(format!("operation \"{}\" unrecognized", op))
    }
}

/*
Some musings about the virtues of using an enum as opposed to using a trait
An enum is simpler to use for my own purposes, and especially if I get to including things like ln(x) and trig fns, 
enums will allow me to be able to easily recognize and simplify expressions like ln(exp(x)) to just x.
*/
#[derive(Debug)]
pub enum Expr{
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>)
}
#[derive(std::cmp::PartialEq)]
enum SegmentorState{
    ExpNum,
    ExpOp
}
struct Segmentor<'a>{
    src: &'a str,
    state: SegmentorState,
    i: usize,
}

impl<'a> Segmentor<'a>{
    pub fn init(src: &'a str) -> Self{
        Self { 
            src: src, 
            state: SegmentorState::ExpNum, 
            i: 0 
        }
    }
    fn inc(&mut self){
        self.i += 1;
    }
}

impl<'a> Iterator for Segmentor<'a>{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.src.len(){
            None
        }
        else{
            let bytes = self.src.as_bytes();
            while bytes[self.i] == ' ' as u8{
                self.i += 1;
            }
            let curr = bytes[self.i];
            if (curr == '(' as u8) | (curr == ')' as u8){ //bitwise '|' instead of logical '||' for efficiency
                let ans = Some(&self.src[self.i..=self.i]);
                self.inc();
                ans
            }
            else if self.state == SegmentorState::ExpNum{
                let mut num_len = 0;
                if curr == '-' as u8{
                    num_len = 1;
                }
                while self.i + num_len < bytes.len() && is_numeric(bytes[self.i + num_len]){
                    num_len += 1;
                }
                let ans = Some(&self.src[self.i..self.i + num_len]);
                self.i += num_len;
                self.state = SegmentorState::ExpOp;
                ans
                
            }
            else{ //self.state == SegmentorState::ExpOp
                let ans = Some(&self.src[self.i..=self.i]);
                self.inc();
                self.state = SegmentorState::ExpNum;
                ans
            }
        }
    }
}


impl Expr{
    pub fn from_str(input: &str) -> Option<Self>{
        if input.len() == 0{
            println!("len was 0?");
            None
        }
        else{
            let v: Vec<&str> = Expr::segment(input);
            Self::from_segments(&v[..])
        }
    }

    pub fn segment(input: &str) -> Vec<&str>{
        let mut ans: Vec<&str> = vec![];
        for thing in Segmentor::init(input){
            ans.push(thing);
        }
        ans
    }

    fn parse_multiple(segs: &[&str]) -> Option<Self>{
        let least_prec_index = Expr::get_least_prec_index(segs)?;
        if least_prec_index < segs.len(){
            let op = segs[least_prec_index];
            let left = Expr::from_segments(&segs[..least_prec_index])?;
            let right = Expr::from_segments(&segs[least_prec_index+1..])?;
            Self::simple_op(left, op, right)
        }
        else{ // if get_least_prec_index returned a number >= segs.len(), then there is an outer layer of parens you need to strip
            Expr::from_segments(&segs[1..(segs.len() - 1)])
        }
    }
    fn get_least_prec_index(segs: &[&str]) -> Option<usize>{
        let get_prec = |index: usize|{
            if index >= segs.len(){
                panic!("wanted precedence at index: {}, full list: {:?}", index, segs)
            }
            match precedence(segs[index]){
                Ok(num) => num,
                Err(msg) => panic!("{}, full list: {:?}", msg, segs)
            }
        };
        let mut least_prec_index = 
        if segs[0] == "("{
            let close_index = match Expr::find_close(segs){
                Ok(num) => num + 1,
                Err(_) => return None
            };
            if close_index < segs.len(){
                close_index
            }
            else {
                return Some(close_index);
            }
        }
        else{
            1
        };
        let mut least_prec = get_prec(least_prec_index);
        let mut i = least_prec_index + 1;
        while i < segs.len(){
            let curr = segs[i];
            if Expr::is_op(curr){
                let prec_i = get_prec(i);
                if prec_i <= least_prec{
                    least_prec_index = i;
                    least_prec = prec_i;
                }
            }
            else if curr == "("{
                i += match Expr::find_close(&segs[i..]){
                    Ok(num) => num,
                    Err(_) => return None
                };
            }
            else{
                let bytes = segs[i].as_bytes();
                if bytes.len() == 0 || !is_number(segs[i]){ //is_number check is probably slow
                    println!("msg from {}: index: {}, tokens: {:?}", line!(), i, segs);
                    return None;
                }
            }
            i += 1;
        }
        Some(least_prec_index)
    }

    pub fn from_segments(segs: &[&str]) -> Option<Self>{
        if is_even(segs.len() as i64){
            println!("tokens: {:?}", segs);
            println!("len even");
            None
        }
        else{
            Self::from_segs(segs)
        }
    }

    fn from_segs(segs: &[&str]) -> Option<Self>{
        debug_assert!(is_odd(segs.len() as i64));
        if segs.len() == 1{
            Self::parse_single(segs)
        }
        else{
            Self::parse_multiple(segs)
        }
    }

    fn _handle_left_parens(segs: &[&str]) -> Option<Self>{
        debug_assert!(segs[0] == "(");
        let close_index = match Expr::find_close(segs){
            Ok(num) => num,
            Err(_) => return None
        };
        if close_index == last_index(segs){
            Self::from_segments(&segs[1..close_index])
        }
        else{
            let left = Self::from_segments(&segs[1..close_index])?;
            let op = segs[close_index + 1];
            let right = Self::from_segments(&segs[close_index + 2..])?;
            Self::simple_op(left, op, right)
        }
    }

    fn _handle_no_left_parens(segs: &[&str]) -> Option<Self>{
        let get_prec = |index: usize|{
            match precedence(segs[index]){
                Ok(num) => num,
                Err(msg) => panic!("{}, full list: {:?}", msg, segs)
            }
        };
        let mut least_prec_index = 1;
        let mut least_prec = get_prec(least_prec_index);
        let mut i = 0;
        while i < segs.len(){
            let curr = segs[i];
            if Expr::is_op(curr){
                let prec_i = get_prec(i);
                if prec_i <= least_prec{
                    least_prec_index = i;
                    least_prec = prec_i;
                }
            }
            else if curr == "("{
                i += match Expr::find_close(&segs[i..]){
                    Ok(num) => num,
                    Err(_) => return None
                };
            }
            else{
                let bytes = segs[i].as_bytes();
                if bytes.len() == 0 || !is_numeric(segs[i].as_bytes()[0]){
                    println!("msg from 'handle_no_left_parens': index: {}, tokens: {:?}", i, segs);
                    return None;
                }
            }
            i += 1;
        }
        let op = segs[least_prec_index];
        let left = Expr::from_segments(&segs[..least_prec_index])?;
        let right = Expr::from_segments(&segs[least_prec_index+1..])?;
        Self::simple_op(left, op, right)
    }

    /**
     * returns the index of the ")" that matches the "(" which should be at the start of the segs
     */
    fn find_close(segs: &[&str]) -> Result<usize, &'static str>{
        debug_assert!(segs[0] == "(");
        let mut parens_open = 1;
        for i in 1..segs.len(){
            if segs[i] == "("{
                parens_open += 1;
            }
            else if segs[i] == ")"{
                parens_open -= 1;
                if parens_open == 0{
                    return Ok(i);
                }
            }
        }
        Err("could not find a matching closeing parenthesis")
    }

    fn parse_single(segs: &[&str]) -> Option<Self>{
        debug_assert!(segs.len() == 1);
        match segs[0].parse::<f64>(){
            Ok(num) => Some(Expr::Num(num)),
            Err(_) => {
                println!("couldn't parse sole token \"{}\"", segs[0]);
                None
            }
        }
    }

    fn simple_op(left: Self, op: &str, right: Self) -> Option<Self>{
        match op{
            "^" => Some(Expr::Pow(Box::new(left), Box::new(right))),
            "*" => Some(Expr::Mul(Box::new(left), Box::new(right))),
            "/" => Some(Expr::Mul(Box::new(left),
                Box::new(Expr::Pow(Box::new(right), Box::new(Expr::Num(-1.0)))))),
            "+" => Some(Expr::Add(Box::new(left), Box::new(right))),
            "-" => Some(Expr::Add(Box::new(left), 
                Box::new(Expr::Mul(Box::new(Expr::Num(-1.0)), Box::new(right))))),
            
            _ => {
                println!("operation \"{}\" unrecognized", op);
                None
            }
        }
    }

    fn is_op(s: &str) -> bool{
        match s{
            "+" | "-" => true,
            "*" | "/" => true,
            "^" => true,
            _ => false
        }
    }

    pub fn eval(&self) -> f64{
        match self{
            Self::Num(ans) => *ans,
            Self::Add(left, right) => 
                left.as_ref().eval() + right.as_ref().eval(),
            Self::Mul(left, right) =>
                left.as_ref().eval() * right.as_ref().eval(),
            Self::Pow(left, right) =>
                f64::powf(left.as_ref().eval(), right.as_ref().eval())
        }
    }
}

impl std::cmp::PartialEq<Expr> for Expr{
    fn eq(&self, other: &Self) -> bool{
        eq_ish(self.eval(), other.eval())
    }

    fn ne(&self, other: &Self) -> bool{
        !(self == other)
    }
}

