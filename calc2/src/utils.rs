pub fn is_num(s: &str) -> bool{
    match s.parse::<f32>(){
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn is_even(n: i64) -> bool{
    n % 2 == 0
}

pub fn is_odd(n: i64) -> bool{
    !is_even(n)
}

pub fn is_digit(c: u8) -> bool{
    '0' as u8 <= c && c <= '9' as u8
}

pub fn eq_ish(a: f64, b: f64) -> bool{
    if about_0(a) | about_0(b){
        about_0(a) & about_0(b)
    }
    else{
        f64::abs(a / b - 1.0) < 0.001
    }
}

pub fn about_0(a: f64) -> bool{
    a.abs() < 0.001
}

pub fn last_index<T>(slice: &[T]) -> usize{
    slice.len() - 1
}