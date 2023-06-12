use std::fmt::Display;

// Define a Node struct for the linked list
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// Define methods for the linked list
impl<T: Display> Node<T> {
    // Create a new node
    fn new(value: T) -> Self {
        Node {
            value,
            next: None,
        }
    }

    // Insert a new node at the end of the list
    fn append(&mut self, value: T) {
		
// Meaning of below code :
/*
 It checks if self.next is Some. If it is, it creates a mutable reference next
 to the value inside Some, allowing you to perform operations on the value.
 
 ref keyword is used to create a reference to the value inside the Some variant of an Option enum.
 The ref keyword allows you to borrow the value rather than taking ownership of it.
*/
        if let Some(ref mut next) = self.next {
            next.append(value);
        } else {
            let new_node = Box::new(Node::new(value));
            self.next = Some(new_node);
        }
    }

    // Print the values in the list
    fn print(&self) {
        println!("{}", self.value);
        if let Some(ref next) = self.next {
            next.print();
        }
    }
}

fn main() {
    // Create a new linked list
    let mut list = Node::new("Om");

    // Append values to the list
	list.append("Namaha");
	list.append("Shivaya");
    list.append("Jai");
    list.append("Shree");
    list.append("Ram");

    // Print the list
    list.print();
}
/*
1
2
3
4
*/