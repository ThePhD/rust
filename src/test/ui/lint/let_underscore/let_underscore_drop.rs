// check-pass
// compile-flags: -W let_underscore_drop

struct NontrivialDrop;

impl Drop for NontrivialDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    let _ = NontrivialDrop; //~WARNING non-binding let on a type that implements `Drop`
}
