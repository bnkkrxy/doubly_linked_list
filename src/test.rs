use crate::DoublyLinkedList;

    fn create_list() -> DoublyLinkedList<i32> {
        let mut list = DoublyLinkedList::new();
        for n in 1..21 {
            list.push_front(n);
        }
        list
    }

    #[test]
    fn test_add_values() {
        let mut list = create_list();

        list.push_front(101);
        list.push_front(102);
        assert_eq!(list.len, 22);

        list.push_back(103);
        assert_eq!(list.len, 23);

        list.add_index(1, 110);
        list.add_index(20, 111);
        assert_eq!(list.len, 25);

        list.add_index(50, 100);
        assert_eq!(list.len, 25);

        println!("{:?}", list);
    }
    
    #[test]
    fn test_len() {
        let mut list = create_list();

        let length = list.len();
        assert_eq!(length, 20)
    }

    #[test]
    fn test_is_empty() {
        let mut list1 = create_list();

        let list2: DoublyLinkedList<i32> = DoublyLinkedList::new();
        
        let empty1 = list1.is_empty();
        let empty2 = list2.is_empty();

        assert_eq!(empty1, false);
        assert_eq!(empty2, true);
    }

    #[test]
    fn test_delete_elements() {
        let mut list = create_list();

        list.pop_front();
        assert_eq!(list.len, 19);

        list.pop_back();
        assert_eq!(list.len, 18);

        list.delete_index(1);
        assert_eq!(list.len, 17);

        println!("{:?}", list);

        let mut list1: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list1.pop_back();
    }

    #[test]
    fn test_value() {
        let mut list1 = create_list();

        let ind = list1.get_by_value(15);
        assert_eq!(ind, Some(5));
    }
    #[test]
    fn test_debug() {
        let list = create_list();
        println!("{:?}", list);
    }
