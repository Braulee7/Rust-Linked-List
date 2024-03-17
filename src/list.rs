use std::cell::RefCell;
use std::rc::Rc;

#[derive (Debug)]
struct Node {
    data: i32,
    next: Option<Link>,
}

// type defintion for link
type Link = Rc<RefCell<Node>>;

#[derive (Debug)]
pub struct List {
    head: Option<Link>,
    tail: Option<Link>,
    size: i32
}

// implementing the List
impl List {
    pub fn build_empty() -> List
    {
        List{head: None, tail: None, size: 0}
    }

    /// adds data to the top of the stack increasing the size
    /// of the list by 1
    pub fn push_front(&mut self, data: i32) -> i32
    {
        match &mut self.head {
            Some(head) => {
                // add new node to head
                let new_node = Some(Rc::new(RefCell::new(Node{data, next:Some(head.clone())})));
                self.head = new_node;
                self.size += 1;
                self.size
            },
            None => {
                // add new node to empty list
                self.head = Some(Rc::new(RefCell::new(Node{data, next: None})));
                self.size += 1;
                // clone it for tail
                self.tail = self.head.as_ref().map(|node| node.clone());
                self.size
            }
        }
    }

    /// adds data to the back of the stack increasing the
    /// list size by 1
    pub fn push_back(&mut self, data: i32) -> i32
    {
        // check if list is empty
        match &mut self.tail {
            Some(tail) => {
                let new_tail = Some(Rc::new(RefCell::new(Node{data, next: None})));
                tail.try_borrow_mut().unwrap().next = new_tail.clone();
                self.tail = new_tail;
                self.size += 1;
                self.size
            },
            None => {
                // add new node to empty list
                self.head = Some(Rc::new(RefCell::new(Node{data, next: None})));
                self.size += 1;
                // clone it for tail
                self.tail = self.head.as_ref().map(|node| node.clone());
                self.size
            }
        }
    }

    /// Takes head off of the linked list returning it's inner data or None
    /// if list was empty
    pub fn pop_front(&mut self) -> Option<i32>
    {
        match &mut self.head {
            Some(head) => {
                let data = head.borrow().data;
                // check if tail is head
                if let Some(tail) = & self.tail {
                    if Rc::ptr_eq(head, tail ) {
                        self.tail = None;
                    }
                }

                // move head forward
                let new_head = head.borrow().next.clone();
                self.head = new_head;
                self.size -= 1;
                Some(data)
            },
            None => None
        }
    }

    /// Takes tail off of list returning its inner data or None 
    /// if list was empty
    pub fn pop_back(&mut self) -> Option<i32> {
        // check if list is empty
        if self.head.is_none() {return None;}

        let mut curr = self.head.as_ref().map(|node| Rc::clone(node));

        while let Some(next) = curr.as_ref().
                                and_then(|node| node.borrow().next.clone()) {
            let next_node = next.clone();
            // checks if next is at tail, should always happen if there's
            // at least 2 elements in the list
            if Rc::ptr_eq(&next_node, &self.tail.as_ref().unwrap()) {
                // Found the second-to-last node
                let data = self.tail.as_ref().unwrap().borrow().data;
                curr.as_ref().unwrap().borrow_mut().next = None;
                self.tail = curr.clone();
                self.size -= 1;
                return Some(data);
            }
            curr = Some(next_node);
        }
        // only one element in list
        let data = self.head.as_ref().unwrap().borrow().data;
        self.head = None;
        self.tail = None;
        self.size -= 1;
        Some(data)
    }

    /// peek looks at the top item on the stack and returns it 
    /// if the list isn't empty, otherwise it returns None
    pub fn peek_front(&self) -> Option<i32>
    {
        match &self.head {
            Some(head) => Some(head.borrow().data),
            None => None
        }
    }

    /// looks at the back of the stack and returns its value
    /// if the list is empty it returns None
    pub fn peek_back(&self) -> Option<i32>
    {
        match &self.tail {
            Some(tail) => Some(tail.borrow().data),
            None => None
        }
    }
}
