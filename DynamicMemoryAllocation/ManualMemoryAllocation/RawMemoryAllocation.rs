/*
Memory allocation , Re-allocation and Deallocation demonstration using std::alloc module.
*/
use std::alloc::{alloc, realloc, dealloc, Layout};
use std::ptr;

fn main() {
    let size = 4; // Example size in bytes
    let align = 8; // Example alignment requirement

    //let layout = Layout::from_size_align(size, align).unwrap();
	 let layout = Layout::new::<u32>(); // Above line is also ok

    let ptr = unsafe { alloc(layout) as *mut u32 };
    if !ptr.is_null() {
        unsafe {
            *ptr = 42;
            println!("Allocated value: {}", *ptr);

            let new_size = 8; // New size in bytes
            let new_layout = Layout::from_size_align(new_size, align).unwrap();

            let new_ptr = realloc(ptr as *mut u8, layout, new_size);

            if !new_ptr.is_null() {
                // Access the reallocated memory
                let new_value = &mut *(new_ptr as *mut u32);
                *new_value = 99;
                println!("Reallocated value: {}", *new_value);

             //   dealloc(new_ptr as *mut u8, new_layout); // if you uncomment this line , leads to double free error 
            }

            dealloc(ptr as *mut u8, layout);
        }
    }
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/DynamicMemoryAllocation$ ./RawMemoryAllocation
Allocated value: 42
Reallocated value: 99
*/