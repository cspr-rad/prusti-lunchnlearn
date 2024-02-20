use prusti_contracts::*; 
struct List {
    val : i32, 
    next : Option<Box<List>>
}

predicate!{
    fn decrements(input: &List, output: &List) -> bool {
        let il = input.len(); 
        if il > 1 {
            il == output.len() + 1
        } else { 
            false
        }
    }
}

impl List {
    // #[pure] // 1
    // #[ensures(result > 0)] // 2
    fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut current = Some(self);
        let max_length = usize::MAX - 1; // Define a maximum length for safety

        while let Some(node) = current {
            // Increment the count for each node
            count += 1;
            if count >= max_length {
                // Prevent going beyond the max_length
                break;
            }
            current = node.next.as_deref();
        }

        count
    }

    // #[pure] // 5
    // #[requires(i < self.len())] // 0
    fn get(&self, i: usize) -> i32 {
        if i == 0 {
            self.val
        } else {
            match self.next {
                Some(ref tail) => tail.get(i - 1),
                None => unreachable!()
            }
        }
    }

    // #[ensures(self.len() == old(self.len()) + 1)] // 3
    // #[ensures(self.get(self.len() - 1) == v)] // 4 
    // #[ensures(forall(|i: usize| i < self.len() - 1 ==> self.get(i) == old(self.get(i))))] // 6
    fn push(&mut self, v: i32) {
        match self.next {
            None => self.next = Some(Box::new(List{val: v, next: None})),
            Some(ref mut tail) => {
                tail.push(v)
            },
        }
    }

    /** don't call on singleton */
    // #[requires(self.len() > 1)] // 6
    // #[ensures(decrements(self, old(self)))] // 7
    pub fn pop(&mut self) -> i32 {
        let next = self.next.take();
        match next {
            Some(mut next_node) => {
                std::mem::swap(self, &mut *next_node);
                self.val
            },
            None => unreachable!()
        }
    }
}

fn main() {
    let mut x = List{val: 2 as i32, next: Some(Box::new(List{val: 1 as i32, next: None}))}; 
    x.push(3); 
    println!("{},{}", x.get(0), x.get(1)); 
    println!("{} pops, leaving {} in zero index", x.pop(), x.get(0));
}
