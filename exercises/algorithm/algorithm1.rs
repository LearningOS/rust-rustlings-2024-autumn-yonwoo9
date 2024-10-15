/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}

impl<T> LinkedList<T>
where
    T: Ord,
{
    /// 合并两个有序链表为一个新的有序链表
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();

        let mut current_a = list_a.start;
        let mut current_b = list_b.start;
        let mut merged_end: Option<NonNull<Node<T>>> = None;

        unsafe {
            // 遍历两个链表，按顺序选择较小的节点
            while let (Some(node_a_ptr), Some(node_b_ptr)) = (current_a, current_b) {
                let node_a = node_a_ptr.as_ref();
                let node_b = node_b_ptr.as_ref();

                let selected_node_ptr;
                let next_node;

                if node_a.val <= node_b.val {
                    selected_node_ptr = node_a_ptr;
                    next_node = node_a.next;
                    current_a = next_node;
                } else {
                    selected_node_ptr = node_b_ptr;
                    next_node = node_b.next;
                    current_b = next_node;
                }

                // 将选中的节点链接到 merged_list
                if merged_list.start.is_none() {
                    merged_list.start = Some(selected_node_ptr);
                } else {
                    if let Some(end_ptr) = merged_end {
                        (*end_ptr.as_ptr()).next = Some(selected_node_ptr);
                    }
                }

                merged_end = Some(selected_node_ptr);
                merged_list.length += 1;
            }

            // 将剩余的节点链接到 merged_list
            while let Some(node_a_ptr) = current_a {
                if merged_list.start.is_none() {
                    merged_list.start = Some(node_a_ptr);
                } else {
                    if let Some(end_ptr) = merged_end {
                        (*end_ptr.as_ptr()).next = Some(node_a_ptr);
                    }
                }
                merged_end = Some(node_a_ptr);
                merged_list.length += 1;
                current_a = (*node_a_ptr.as_ptr()).next;
            }

            while let Some(node_b_ptr) = current_b {
                if merged_list.start.is_none() {
                    merged_list.start = Some(node_b_ptr);
                } else {
                    if let Some(end_ptr) = merged_end {
                        (*end_ptr.as_ptr()).next = Some(node_b_ptr);
                    }
                }
                merged_end = Some(node_b_ptr);
                merged_list.length += 1;
                current_b = (*node_b_ptr.as_ptr()).next;
            }

            // 设置 merged_list 的 end 指针
            merged_list.end = merged_end;
        }

        // 清空 list_a 和 list_b 以避免析构时重复释放节点
        list_a.start = None;
        list_a.end = None;
        list_a.length = 0;

        list_b.start = None;
        list_b.end = None;
        list_b.length = 0;

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
