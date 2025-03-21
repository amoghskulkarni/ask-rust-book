fn main() {
    // return_string();
    // name_with_title();
}

// Data doesn't live long enough
fn return_string() -> &String {
    let s = String::from("hello");
    &s
}

// Data cannot have an immutable reference while having a mutable reference
// Not a good practice to change the input parameter within a function
//
// Could we take the ownership of the input parameter? (v2)
// Also not a good practice to take the ownership of the input parameter,
// because the caller may need the input parameter after the function call
//
// Good choice is to return the modified data from the function (v3)
fn name_with_title() {
    let mut name = vec![String::from("John"), String::from("Doe")];
    let name_i = vec![String::from("John"), String::from("Doe")];
    // let first = &name[0];
    let full = stringify_name_with_title(&mut name);
    let full2 = stringify_name_with_title_v2(name);
    let full3 = stringify_name_with_title_v3(&name_i);
    println!("Full name: {}", full);
}
fn stringify_name_with_title(n: &mut Vec<String>) -> String {
    n.push(String::from("Esq."));
    let full = n.join(" ");
    full
}
fn stringify_name_with_title_v2(mut n: Vec<String>) -> String {
    n.push(String::from("Esq."));
    let full = n.join(" ");
    full
}
fn stringify_name_with_title_v3(n: &Vec<String>) -> String {
    let mut name_clone = n.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}
