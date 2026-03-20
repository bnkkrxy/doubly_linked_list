use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

struct Node<T> {
    value: T,
    ref_next: Option<Rc<RefCell<Node<T>>>>,
    ref_prev: Option<Weak<RefCell<Node<T>>>>,
}

pub struct DoublyLinkedList<T> {
    head: Option<T>,
    tail: Option<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    fn push_front() {} //добавление элемента в начало

    fn push_back() {} //добавление элемента в конец 

    fn pop_front() {} //удаление элемента с начала

    fn pop_back() {} //удаление элемента с конца 

    fn add_index() {} //вставка элемента по индексу

    fn delete_index() {} //удаление элемента по индексу

    fn search_value() {} //поиск элемента по значению
}








#[cfg(test)]
mod tests {
    use super::*;

}
