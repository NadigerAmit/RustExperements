
trait bhakt {
    fn chant(&self);
}

struct shiva_bhakt;
struct vishnu_bhakt;

impl bhakt for shiva_bhakt {
    fn chant(&self) {
        println!("Om Namah Shivaya");
    }
}
impl bhakt for vishnu_bhakt {
    fn chant(&self) {
        println!("Om Namah Narayanaya");
    }
}
fn main() {
    let mut v_bhakt:Vec<Box<dyn bhakt>> = Vec::new();
    v_bhakt.push(Box::new(vishnu_bhakt));
    v_bhakt.push(Box::new(shiva_bhakt));
    for i in v_bhakt {
        i.chant();
    }
}