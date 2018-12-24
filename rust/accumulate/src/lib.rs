pub fn map<F, T, S>(input: Vec<T>, mut function: F) -> Vec<S>
    where F: FnMut(T) -> S 
{
    let mut vec = Vec::with_capacity(input.len());
    for item in input {
        vec.push(function(item));
    }
    vec
}
