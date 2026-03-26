use std::fmt::Error;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;
pub mod list_iterator;
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { 
            value, 
            next: None, 
            prev: None 
        }))
    }
}

impl<T> DoublyLinkedList<T> {

    pub fn new() -> Self {
        DoublyLinkedList { 
            head: None, 
            tail: None, 
            len: 0 
        }
    }

    pub fn push_front(&mut self, value: T){  //добавление элемента в начало
        let new_node = Node::new(value);

        match self.head.take() {
            Some(ex_head) => {
                ex_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(ex_head);

                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
        }

        self.len += 1;
        
    } 

    fn push_back(&mut self, value: T) { //добавление элемента в конец 
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(ex_tail) => {
                new_node.borrow_mut().prev = Some(Rc::downgrade(&ex_tail));
                ex_tail.borrow_mut().next = Some(Rc::clone(&new_node));

                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }

        self.len += 1;
    }

    fn pop_front(&mut self) { //удаление элемента с начала

        if let Some(old_head) = self.head.take() {
        
            self.len -= 1;
            let mut old_node = old_head.borrow_mut();
                
            match old_node.next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }

        }
    } 

    fn pop_back(&mut self) { //удаление элемента с конца 
        if let Some(old_tail) = self.tail.take() {
        
            self.len -= 1;
            let mut old_node_weak = old_tail.borrow_mut().prev.take();
                
            match old_node_weak {
                Some(new_tail) => {
                    if let Some(new_tail_rc) = new_tail.upgrade() {
                        new_tail_rc.borrow_mut().next = None;
                        self.tail = Some(new_tail_rc);
                    }
                }
                None => {
                    self.head = None;
                }
            }

        }
    }

    fn get_node(&self, index: usize) -> Option<Rc<RefCell<Node<T>>>> { //для поиска ноды
        if index > self.len {
            return None;
        }

        let mut curr = self.head.clone();
        let mut i = 0;

        while let Some(node) = curr {
            if i == index {
                return Some(node);
            }
            curr = node.borrow().next.clone();
            i += 1;
        }

        None

    }

    //с итераторами эти
    fn add_index(&mut self, index: usize, value: T) -> Result<(), String> { //вставка элемента по индексу
        if index == 0 {
            self.push_front(value);
            return Ok(())
        }
        if index == self.len {
            self.push_back(value);
            return Ok(())
        }

        if let Some(curr_node) = self.get_node(index) {
            let new_node = Node::new(value);
            new_node.borrow_mut().next = Some(Rc::clone(&curr_node));
            
            let old_prev = curr_node
                .borrow()
                .prev
                .as_ref()
                .and_then(|weak_ref| weak_ref.upgrade());
            if let Some(old_prev_rc) = old_prev {
                old_prev_rc.borrow_mut().next = Some(Rc::clone(&new_node));
            }
        }           

        self.len += 1;

        Ok(())
    }

    fn delete_index() { //удаление элемента по индексу

    }

    fn search_value() { //поиск элемента по значению

    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}








#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut list = DoublyLinkedList::new();
        list.push_front(10);
        list.push_front(20);

        assert_eq!(list.len, 2);
    }
    #[test]
    fn test_push_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(40);
        list.push_back(30);
        list.push_back(8);


        assert_eq!(list.len, 3);
    }
    #[test]
    fn test_len() {
        let mut list = DoublyLinkedList::new();
        list.push_back(40);
        list.push_back(30);
        list.push_front(8);

        let length = list.len();
        assert_eq!(length, 3)
    }

    #[test]
    fn test_is_empty() {
        let mut list1 = DoublyLinkedList::new();
        list1.push_back(40);
        list1.push_back(30);
        list1.push_front(8);

        let list2: DoublyLinkedList<i32> = DoublyLinkedList::new();
        
        let empty1 = list1.is_empty();
        let empty2 = list2.is_empty();

        assert_eq!(empty1, false);
        assert_eq!(empty2, true);
    }

    #[test]
    fn test_pop_front() {
        let mut list1 = DoublyLinkedList::new();
        list1.push_back(40);
        list1.push_back(30);
        list1.push_front(8);

        list1.pop_front();

        assert_eq!(list1.len, 2)
    }

    #[test]
    fn test_pop_back() {
        let mut list1 = DoublyLinkedList::new();
        list1.push_back(40);
        list1.push_back(30);
        list1.push_front(8);

        list1.pop_back();

        assert_eq!(list1.len, 2)
    }


}
