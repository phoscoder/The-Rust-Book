
pub trait Iterator {
    type Item; 

    fn next(&mut self) -> Option<Self::Item>;
}


#[derive(PartialEq, Debug)]
struct Show {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: uu32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}


fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();


    

    for value in v1_iter {
        println!("Got: {}", value);
    }

    // Iterators are lazy, so we have to consume the iterator to see any effect
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2: {:?}", v2);


}
