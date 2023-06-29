/*
Objective is it to create the array dynamically on the heap 
1. get the SIZE from user 
2. Allocate the array on the heap of SIZE 
3. Update the array
4. Read the array.
*/
use std::io;

fn main() {
    // Read the size of the array from the user
    println!("Enter the size of the array:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input string to get the size
    let size: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default size of 0.");
            0
        }
    };

    // Allocate the array on the heap
    let mut array: Box<[i32]> = vec![0; size].into_boxed_slice();

/*
//    Below is another way to create the aray on the heap without using the 
// into_boxed_slice() function.
    // Allocate the array on the heap
    let mut vec = Vec::with_capacity(size);
    let array: &mut [i32] = &mut vec;
*/
    // Access and modify elements of the array
    for i in 0..size {
        array[i] = (i + 1) as i32;
    }

    // Print the contents of the array
    for i in 0..size {
        println!("Element {}: {}", i, array[i]);
    }
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/DynamicMemoryAllocation$ ./DynamicMemoryAlloc
Enter the size of the array:
8
Element 0: 1
Element 1: 2
Element 2: 3
Element 3: 4
Element 4: 5
Element 5: 6
Element 6: 7
Element 7: 8
*/