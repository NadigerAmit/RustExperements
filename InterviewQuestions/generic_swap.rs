use std::mem;
#[derive(Clone,Debug,Copy)]
struct point<T> {
    x:T,
	y:T,
}

fn swap<T:Clone>(a:&mut T,b:&mut T) {
    let mut temp:T = a.clone();
	*a = b.clone();
	*b = temp;
}

fn swap_with_mem_swap<T:Clone>(a:&mut T,b:&mut T) {
    std::mem::swap(a,b);
}

fn main() {
    let mut p1:point<u32> = point{x:1,y:2};
	let mut p2:point<u32> = point{x:100,y:200};
	println!("Before swap p1 = {:?} and p2 = {:?}",p1,p2);
	swap(&mut p1,&mut p2);
	println!("After 1st swap p1 = {:?} and p2 = {:?}",p1,p2);
	swap_with_mem_swap(&mut p1,&mut p2);
	println!("After 2nd swap p1 = {:?} and p2 = {:?}",p1,p2);
}
/*
Op -> 
Before swap p1 = point { x: 1, y: 2 } and p2 = point { x: 100, y: 200 }
After 1st swap p1 = point { x: 100, y: 200 } and p2 = point { x: 1, y: 2 }
After 2nd swap p1 = point { x: 1, y: 2 } and p2 = point { x: 100, y: 200 }
*/