
//  
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetimes tell the borrow check that the references passed to a function are valid for the lifetime 'a'
// and also the returned reference is valid for the lifetime 'a'
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// &i32 - a reference to an i32 value
// &'a i32 - a reference to an i32 value with lifetime 'a'
// &'a mut i32 - a mutable reference to an i32 value with lifetime 'a'

fn main() {

    let str1 = String::from("hello");
    let str2 = String::from("world");
    
    let result = longest(str1.as_str(), str2.as_str());
    println!("The longest string is {}", result);
}
