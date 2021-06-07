use std::fmt::Display;

pub fn vec_a_haskell_vec<T: Display>(vec:&Vec<T>)->String{
    let mut vector = String::from("[");
    for elem in vec{
        vector.push_str(&format!("{}",elem));
        vector.push(',');
    }
    vector.pop();
    vector.push(']');
    vector
}

pub fn haskell_vec_a_vec<T: std::str::FromStr>(haskell_vec: String)->Result<Vec<T>,T::Err>{
    let mut vector = Vec::new();
    let haskell_vec = haskell_vec
        .trim()
        .trim_end_matches("]")
        .trim_start_matches("[")
        .split(",");
    for val in haskell_vec{
        vector.push(val.parse::<T>()?);
    }
    Ok(vector)
}

pub fn tupla_a_haskell_tupla<T: Display,R: Display>(tupla: &(T,R))->String{
    format!("({},{})",tupla.0,tupla.1)
}