fn main() {
    let array = vec![1,9,-34,2,0];
    let mut largestNum = array[0];
    for i in array.iter() {
    	if i> &largestNum {
    		largestNum = *i;
    	}
    }
	println!("largest num = {}",largestNum); // 9
}
