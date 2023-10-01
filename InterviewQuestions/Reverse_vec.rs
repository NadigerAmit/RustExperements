fn main() {
    let mut v:Vec<i32> = Vec::new();
    let mut v1:Vec<i32> = vec!(1,2,3,4,5,6);
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    reverseVector(&mut v);
    reverseVector(&mut v1);
    println!("Reverse vector V = {:?}",v);
    println!("Reverse vector V1 = {:?}",v1);
}

fn reverseVector(v:&mut Vec<i32>) {
    let mut len = v.len() ; 
    for i in 0..v.len() {
    	if i >=len-1 {
    	    break;
    	}
    	// below is swap logic
	let mut t = v[len-1];
	v[len-1]= v[i];
	v[i] = t;
    	len = len -1;
    }
}

/*
Op =>
Reverse vector V = [4, 3, 2, 1]
Reverse vector V1 = [6, 5, 4, 3, 2, 1]
*/