trait Shape {
    fn area(&self) -> f64;
}

trait Drawable {
    fn draw(&self);
}


struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}.", self.radius);
    }
}

struct Square {
    side_length: f64,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side length {}.", self.side_length);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area(shape: &dyn Shape) {
    println!("The area of the shape is {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 10.0, height: 5.0 };
    print_area(&circle);
    print_area(&rectangle);
	
	let shapes: Vec<&dyn Drawable> = vec![
        &Circle { radius: 1.0 },
        &Square { side_length: 2.0 },
    ];

    for shape in &shapes {
        shape.draw();
    }
}