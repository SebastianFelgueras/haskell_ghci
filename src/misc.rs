pub fn vec_a_haskell_vec<T: std::fmt::Display>(vec:&Vec<T>)->String{
    let mut vector = String::from("[");
    for elem in vec{
        vector.push_str(&format!("{}",elem));
        vector.push(',');
    }
    vector.pop();
    vector.push(']');
    vector
}