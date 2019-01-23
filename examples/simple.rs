#[macro_use]
extern crate objekt_clonable;

extern crate objekt;

#[clonable]
trait MyTrait: Clone {
    fn recite(&self);
}

impl MyTrait for String {
    fn recite(&self) {
        println!("{} ♫", self);
    }
}

#[derive(Clone)]
struct Container {
    trait_object: Box<MyTrait>
}

fn main() {
    let line = "The slithy structs did gyre and gimble the namespace";

    // Build a trait object holding a String.
    // This requires String to implement MyTrait and std::clone::Clone.
    let x: Box<MyTrait> = Box::new(String::from(line));

    x.recite();

    // The type of x2 is a Box<MyTrait> cloned from x.
    let x2 = objekt::clone_box(&*x);

    x2.recite();
}
