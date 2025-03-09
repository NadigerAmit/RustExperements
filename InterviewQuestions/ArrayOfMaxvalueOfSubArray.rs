/*
This problem was asked by Google.

Given an array of integers and a number k, where 1 <= k <= length of the array, compute the maximum values of each subarray of length k.

For example, given array = [10, 5, 2, 7, 8, 7] and k = 3, we should get: [10, 7, 8, 8], since:

10 = max(10, 5, 2)
7 = max(5, 2, 7)
8 = max(2, 7, 8)
8 = max(7, 8, 7)
Do this in O(n) time and O(k) space. You can modify the input array in-place and you do not need to store the results.
You can simply print them out as you compute them.

*/


fn getMaxOfThree(a:i32,b:i32,c:i32)->i32 {
    if a>b {
        if a>c {
            return a;
        }
    } else {
        if b>c {
            return b;
        }
    }
    return c;
}

fn main() {
    let v = vec![10, 5, 2, 7, 8, 7];
    let mut r:Vec<i32> = vec![];
    for i in 0..v.len() {
        if i+1>v.len()-1 ||  i+2>v.len()-1 {
           println!("Result vec = {:?}",r);
           return ;
        }
        let n1 = v[i];
        let n2 = v[i+1];
        let n3 = v[i+2];
        let m = getMaxOfThree(n1 as i32 ,n2 as i32 ,n3 as i32);
        r.push(m);
    }
}
/*
Result vec = [10, 7, 8, 8]
*/