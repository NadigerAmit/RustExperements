// Implement stack and also which can return min element in the stack

struct s {
     items:Vec<i32>,
     max_num:usize,
     min_v:Vec<i32>,
}

impl s {
    fn new(m:usize) -> Self{
        s{
            items:Vec::new(),
            min_v:Vec::new(),
            max_num:m,
        }
    }
    fn push(&mut self,item:i32) {
        if self.items.len() < self.max_num {
            self.items.push(item);
            if self.min_v.is_empty() {
                self.min_v.push(item);
            } else if self.min_v[self.min_v.len()-1] > item {
                self.min_v.push(item);
            }
        }
        
    }
    fn pop(&mut self) {
        if !self.items.is_empty() {
            let item = self.items.pop();
            if item.unwrap() == self.min_v[self.min_v.len()-1] {
                self.min_v.pop();
            }
        }
    }
    fn top(&self)->i32 {
        self.items[self.items.len()-1]
    }
    fn min(&self)->i32 {
        self.min_v[self.min_v.len()-1]
    }
}

fn main() {
    println!("Jai Shree Ram");
    let mut st:s = s::new(5);
    st.push(43);
    st.push(100);
    st.push(34);
    st.push(2);
    st.push(23);
    st.pop();
    st.pop();
    st.push(24);
    println!("items of stack {:?}",st.items);
    println!("Min items of stack {:?}",st.min_v);

}