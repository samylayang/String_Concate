fn main() {
    let s1 = "Hello";
    let s2 = "World";
    // You need to convert &str to String before concatenating
    let result = s1.to_string() + " " + s2;
    println!("{}", result); // Output: "Hello World"

    let s1 = "Hello";
    let s2 = "World";
    // Using format! macro for concatenation
    let result = format!("{} {}", s1, s2);
    println!("{}", result); // Output: "Hello World"

    let s1 = "Hello";
    let s2 = "World";
    // Using push_str method for concatenation
    let mut result = s1.to_string();
    result.push_str(" ");
    result.push_str(s2);
    println!("{}", result); // Output: "Hello World"
}
