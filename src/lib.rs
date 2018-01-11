extern crate ggez;
extern crate rand;

pub mod components;
pub mod graphics;

pub fn main() {
    println!("Hello world invaders");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
