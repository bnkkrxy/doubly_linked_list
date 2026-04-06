use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;
use std::fmt;

use crate::list_iterator::ListIterator;
pub mod list_iterator;
mod test;

pub struct Node<T> {
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

impl<T: Clone + PartialEq> IntoIterator for &DoublyLinkedList<T> {
    type Item = T;
    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: fmt::Debug + PartialEq + Clone> fmt::Debug for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq + Clone> DoublyLinkedList<T> {

    pub fn new() -> Self {
        DoublyLinkedList { 
            head: None, 
            tail: None, 
            len: 0 
        }
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator {
            current: self.head.clone(),
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

    pub fn push_back(&mut self, value: T) { //добавление элемента в конец 
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

    pub fn pop_front(&mut self) -> Option<T>{ //удаление элемента с начала
        self.head.take().map(|old_head| {
            self.len -= 1;
            let next_node = old_head.borrow_mut().next.take();
                
            match next_node {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }

            match Rc::try_unwrap(old_head) {
                Ok(ref_cell) => ref_cell.into_inner().value,
                Err(_) => panic!("Node is still borrowed!")
            }
        })
    } 

    pub fn pop_back(&mut self) -> Option<T> { //удаление элемента с конца 
        self.tail.take().map(|old_tail| {
            self.len -= 1;
            let prev_node_rc = old_tail
                .borrow_mut()
                .prev
                .take()
                .and_then(|weak| weak.upgrade());
                
            match prev_node_rc {
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }

            match Rc::try_unwrap(old_tail) {
                Ok(ref_cell) => ref_cell.into_inner().value,
                Err(_) => panic!("Node is still borrowed!")
            }
        })
    }

    pub fn get_node(&self, index: usize) -> Option<Rc<RefCell<Node<T>>>> { //для поиска ноды
        if index >= self.len {
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

    pub fn add_index(&mut self, index: usize, value: T) -> Result<(), String> { //вставка элемента по индексу
        
        if index > self.len {
            return Err("Index out of list!".to_string());
        }
        
        if index == 0 {
            self.push_front(value);
        }
        else if index == self.len {
            self.push_back(value);
        }
        else {
            if let Some(curr_node) = self.get_node(index) {
                let new_node = Node::new(value);
                new_node.borrow_mut().next = Some(Rc::clone(&curr_node));
                
                let old_prev_weak = curr_node.borrow().prev.clone();
                new_node.borrow_mut().prev = old_prev_weak.clone();

                if let Some(old_prev_rc) = 
                    old_prev_weak.and_then(|weak_ref| weak_ref.upgrade()) {
                        old_prev_rc.borrow_mut().next = Some(Rc::clone(&new_node));
                }

                curr_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));

                self.len += 1;
            } else {
                return Err("Internal error: node not found".to_string());
            }          
        }
        Ok(())
    }

    pub fn delete_index(&mut self, index: usize) -> Result<T, String> { //удаление элемента по индексу
        
        if index >= self.len {
            return Err("Index out of list!".to_string());
        }
        if index == 0 {
            return self.pop_front().ok_or("List is empty!".to_string());
        }
        if index == self.len - 1 {
            return self.pop_back().ok_or("List is empty!".to_string());
        }

        let curr_node = self.get_node(index).ok_or("Node not found")?;
        let prev_deleted = curr_node
            .borrow()
            .prev
            .as_ref()
            .clone()
            .and_then(|weak_ref| weak_ref.upgrade());
        let next_deleted = curr_node
            .borrow()
            .next
            .clone();

        if let Some(prev_deleted_rc) = prev_deleted.as_ref() {
            prev_deleted_rc.borrow_mut().next = next_deleted.clone();
        }
        else {
            self.head =  next_deleted.clone();
        }

        if let Some(next_deleted_rc) = next_deleted.as_ref() {
            next_deleted_rc.borrow_mut().prev = prev_deleted.as_ref().map(|rc| Rc::downgrade(rc));
        }
        else {
            self.tail = prev_deleted.clone();
        }

        self.len -= 1;

        match Rc::try_unwrap(curr_node) {
            Ok(ref_cell) => Ok(ref_cell.into_inner().value),
            Err(_) => panic!("Node is still borrowed!")
        }
    }
        


    pub fn get_by_value(&self, value: T) -> Option<usize> { //поиск элемента по значению
        let mut curr = self.head.clone();
        let mut index = 0;

        while let Some(node_rc) = curr.as_ref() {
            if node_rc.borrow().value == value {
                return Some(index);
            }

            let next_node = node_rc.borrow().next.clone();
            curr = next_node;
            index += 1;
        }

        None
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}