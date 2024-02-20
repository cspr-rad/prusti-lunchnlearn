struct List {
    val : i32, 
    next : Option<Box<List>>
}

impl List {
    fn len(&self) -> usize {
        match self.next {
            None => 1,
            Some(ref tail) => 1 + tail.len(),
        }
    }
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
    fn push(&mut self, v: i32) {
        match self.next {
            None => self.next = Some(Box::new(List{val: v, next: None})),
            Some(ref mut tail) => {
                tail.push(v)
            },
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        let next = self.next.take();
        match next {
            Some(mut next_node) => {
                std::mem::swap(self, &mut *next_node);
                Some(self.val)
            },
            None => {
                let val = self.val;
                self.val = 0; 
                Some(val)
            }
        }
    }
}

fn main() {
    let mut x = List{val: 2 as i32, next: Some(Box::new(List{val: 1 as i32, next: None}))}; 
    x.push(3); 
    println!("{},{}", x.get(0), x.get(1)); 
    println!("{} pops, leaving {} in zero index", x.pop().unwrap(), x.get(0));
}
