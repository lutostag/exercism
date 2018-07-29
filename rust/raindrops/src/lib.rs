pub fn raindrops(n: usize) -> String {
    let mut raindrop = String::new();
    if n % 3 == 0 {
        raindrop += "Pling";
    }
    if n % 5 == 0 {
        raindrop += "Plang";
    }
    if n % 7 == 0 {
        raindrop += "Plong";
    }
    if raindrop == "" {
        return n.to_string();
    }
    return raindrop;
}
