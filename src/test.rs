use crate::DoublyLinkedList;

    fn create_list() -> DoublyLinkedList<i32> { //лист на 5 элементов [1, 2, 3, 4, 5]
        let mut list = DoublyLinkedList::new();
        for n in 1..6{
            list.push_back(n);
        }
        list
    }

    fn to_vec<T: Clone + PartialEq>(list: &DoublyLinkedList<T>) -> Vec<T> { //преобразование в вектор
        list.iter().collect()
    }

    #[test]
    fn test_push_front_back() {
        let mut list = create_list();

        list.push_front(101);
        assert_eq!(list.len, 6);
        assert_eq!(to_vec(&list), vec![101, 1, 2, 3, 4, 5]);

        list.push_back(103);
        assert_eq!(list.len, 7);
        assert_eq!(to_vec(&list), vec![101, 1, 2, 3, 4, 5, 103]);
    }

    #[test]
    fn test_add_index() {
        let mut list = DoublyLinkedList::new();
        list.add_index(0, 1).unwrap();
        list.add_index(1, 3).unwrap();
        list.add_index(1, 2).unwrap();

        assert_eq!(list.len, 3);
        assert_eq!(to_vec(&list), vec![1, 2, 3]);

        assert!(list.add_index(10, 100).is_err());
    }
    
    #[test]
    fn test_len() {
        let list = create_list();

        let length = list.len();
        assert_eq!(length, 5)
    }

    #[test]
    fn test_is_empty() {
        let list1 = create_list();

        let list2: DoublyLinkedList<i32> = DoublyLinkedList::new();
        
        let empty1 = list1.is_empty();
        let empty2 = list2.is_empty();

        assert_eq!(empty1, false);
        assert_eq!(empty2, true);
    }

    #[test]
    fn test_delete_elements() {
        let mut list = create_list();
        assert_eq!(list.pop_front(), Some(1)); // [2, 3, 4, 5]
        assert_eq!(list.pop_back(), Some(5));  // [2, 3, 4]
        assert_eq!(list.pop_front(), Some(2)); // [3, 4]
        assert_eq!(list.pop_back(), Some(4)); // [3]
        assert_eq!(list.pop_front(), Some(3)); // []
        
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_delete_index() {
        let mut list = create_list();

        let val = list.delete_index(2).unwrap();
        assert_eq!(val, 3);
        assert_eq!(to_vec(&list), vec![1, 2, 4, 5]);

        // удаление головы
        assert_eq!(list.delete_index(0).unwrap(), 1); // [2, 4, 5]

        // удаление хвоста
        assert_eq!(list.delete_index(2).unwrap(), 5); // [2, 4]

        assert_eq!(list.delete_index(1).unwrap(), 4); // [2]
        assert_eq!(list.len(), 1);
        
        assert_eq!(list.delete_index(0).unwrap(), 2);
        assert!(list.is_empty());
    }

    #[test]
    fn test_get_value() {
        let list = create_list();
        assert_eq!(list.get_by_value(5), Some(4));
        assert_eq!(list.get_by_value(100), None);
    }

    #[test]
    fn test_iterator_integrity() {
        let mut list = create_list(); 
        list.delete_index(2).unwrap(); 
        list.pop_back();
        list.pop_front(); // [2, 4]
        list.add_index(1, 100).unwrap(); // [2, 100, 4]

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(100));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);

        assert_eq!(list.len, 3);
    }

    #[test]
    fn test_debug() {
        let list = create_list();
        println!("{:?}", list);
    }

    #[test]
    fn test_empty_list_operations() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
        assert!(list.delete_index(0).is_err());
        assert_eq!(list.get_by_value(10), None);
        assert_eq!(list.iter().next(), None);
    }
