use objekt_clonable::*;


#[clonable]
trait MyTrait: std::clone::Clone {
    fn recite(&self);
}

#[clonable]
trait MyTrait2: std::clone::Clone {
    fn recite2(&self);
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
