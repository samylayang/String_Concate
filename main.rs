fn main() {
    let s1 = "Hello";
    let s2 = "World";

    // You need to convert &str to String before concatenating
    let result = s1.to_string() + " " + s2;
    println!("{}", result); // Output: "Hello World"
}
