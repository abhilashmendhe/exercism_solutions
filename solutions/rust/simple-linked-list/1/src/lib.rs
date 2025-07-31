use std::ptr;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: *mut Node<T>
}

impl <T> Node<T> {
    fn new(value: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Node {
            value,
            next: ptr::null_mut()
        }))
    }
}
#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    // pub head: Option<Box<Node<T>>>,
    pub head: *mut Node<T>, 
    pub size: usize
}


impl<T: std::fmt::Debug> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: ptr::null_mut(),
            size: 0
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        if self.size == 0 {
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, _element: T) {
        unsafe {
            let new_node = Node::new(_element);

            if self.head.is_null(){
                self.head = new_node;
            } else {
                let old_node = self.head;
                self.head = new_node;
                (*self.head).next = old_node;
            }
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size != 0 {
            self.size -= 1;
        }
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let f_node = Box::from_raw(self.head);
                self.head = f_node.next;
                Some(f_node.value)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe {
            // let what = self.head.as_ref().map(|f| &f.value);
            if let Some(ref boxed_node) = self.head.as_ref() {
                Some(&boxed_node.value)
            } else {
                None
            }
        }
    }

    #[must_use]
    pub fn rev(&mut self) -> SimpleLinkedList<T> {
        let mut new_list = SimpleLinkedList::new();
        while let Some(value) = self.pop() {
            new_list.push(value);
        }
        new_list
    }

    pub fn print_list(&mut self) {
        unsafe {
            let mut t_node = self.head;

            while !t_node.is_null() {
                
                print!("{:?} -> ",(*t_node).value);
                t_node = (*t_node).next;
            }
            println!();
        }
    }
}

impl<T: std::fmt::Debug> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut i = _iter.into_iter();
        let mut list= SimpleLinkedList::new();
        while let Some(data) = i.next() {
            list.push(data);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: std::fmt::Debug + Copy> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v:Vec<T> = vec![];
        unsafe {
            let mut t_node = _linked_list.head;

            while !t_node.is_null() {
                v.insert(0,(*t_node).value);
                t_node = (*t_node).next;
            }
        }
        v
    }
}
