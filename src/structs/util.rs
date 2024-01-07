pub fn fmin<T>(a: T, b: T) -> T 
    where T: Into<f64> + Copy
{
    if a.into() < b.into() { a } else { b }
}

pub fn fmax<T>(a: T, b: T) -> T 
    where T: Into<f64> + Copy
{
    if a.into() > b.into() { a } else { b }
}