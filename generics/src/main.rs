

// Point struct with generic type parameters T and U.
// x and y can be of the same type or different types.
struct Point<T, U> {
    x: T,
    y: U,
}

// impl for Point with T and U types.
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

}

// impl for Point with T and f64 types.
impl<T> Point<T, f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}


// This function returns the largest value in a vector of generic type T.
// Where T must implement PartialOrd and Copy traits.
fn get_largest<T: PartialOrd + Copy>(values: Vec<T>) -> T {
    let mut largest = values[0];
    for value in values {
        if value > largest {
            largest = value;
        }
    }
    largest
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    println!("p1.x = {}", p1.x());
    println!("p1.y = {}", p1.y());
    println!("p2.x = {}", p2.x());

    let numbers = vec![34, 50, 25, 100, 65];
    let largest_number = get_largest(numbers);
    println!("The largest number is {}", largest_number);

    let chars = vec!['y', 'm', 'a', 'q'];
    let largest_char = get_largest(chars);
    println!("The largest char is {}", largest_char);
}


