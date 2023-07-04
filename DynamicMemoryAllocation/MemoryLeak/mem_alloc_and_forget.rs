use std::mem;

/// Allocate memory into the module's linear memory
/// and return the offset to the start of the block.
fn alloc_and_forget(len: usize) -> *mut u8 {
    // create a new mutable buffer with capacity `len`
    let mut buf = Vec::with_capacity(len);
    // take a mutable pointer to the buffer
    let ptr = buf.as_mut_ptr();
    // prevent the buffer from being deallocated when it goes out of scope
    mem::forget(buf);
    // return the pointer so the runtime can write data at this offset
    ptr
}

fn main() {
    // Allocate a memory block of size 10
    let ptr = alloc_and_forget(10);
    
    unsafe {
        // Dereference the pointer and write values to the allocated memory block
        for i in 0..10 {
            *ptr.add(i) = i as u8;
        }
    }
    
    // Read the values from the allocated memory block
    let values = unsafe { std::slice::from_raw_parts(ptr, 10) };
    println!("{:?}", values);
    
    // Deallocate the memory block manually
    unsafe {
        Vec::from_raw_parts(ptr, 10, 10);
    }
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP$ ./mem_alloc_and_forget
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
*/

/*
Checking for the memoryy leak 

amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP$ valgrind --tool=memcheck ./mem_alloc_and_forget
==2758== Memcheck, a memory error detector
==2758== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==2758== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==2758== Command: ./mem_alloc_and_forget
==2758==
==2758== error calling PR_SET_PTRACER, vgdb might block
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
==2758==
==2758== HEAP SUMMARY:
==2758==     in use at exit: 0 bytes in 0 blocks
==2758==   total heap usage: 12 allocs, 12 frees, 6,263 bytes allocated
==2758==
==2758== All heap blocks were freed -- no leaks are possible
==2758==
==2758== For lists of detected and suppressed errors, rerun with: -s
==2758== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
*/
/*
The line Vec::from_raw_parts(ptr, 10, 10); in the code snippet is used to deallocate the memory block that was previously allocated using alloc.

In Rust, the Vec::from_raw_parts function is a safe way to convert a raw pointer, along with its capacity and length, back into a Vec. It takes three arguments:

ptr: A raw pointer to the memory block.
length: The length of the memory block, which represents the number of elements in the block.
capacity: The capacity of the memory block, which represents the total number of elements that the block can hold without reallocation.
By calling Vec::from_raw_parts(ptr, 10, 10);, we create a Vec<u8> from the raw pointer ptr and specify that the length and capacity of the vector are both 10. This allows the Vec to take ownership of the memory block.

When the Vec goes out of scope, it will automatically deallocate the memory block for us, ensuring proper memory management and preventing memory leaks.

Note that using Vec::from_raw_parts is considered safe because it guarantees that the deallocation is performed correctly, taking into account the original capacity and length of the memory block. 
This avoids any potential undefined behavior or memory corruption that could occur if we were to deallocate the memory manually without considering the original capacity and length.

*/