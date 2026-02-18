pub fn reverse(input: &str) -> String {
    

    let to_reverse : Vec<char> = input.chars().collect();

    let length = to_reverse.len();
    let mut reversed : String = String::new();
    for i in 0..length {
        reversed.push(to_reverse[length-i-1]);
    }
    
    reversed
    
}
