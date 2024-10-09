fn main() {
    let cat = ("Furry McFurson", 3.5, "qwer ");

    // Destructuring the tuple.
    let (name, age, qwet) = cat;

    println!("{} is {} years old {}",name,age,qwet);
}
