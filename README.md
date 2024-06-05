# Rust 笔试题

## 1 请阐述你对 Rust 所有权系统的理解，可从以下角度

A. 单一所有权，
B. 内存管理方式，
C. Copy/Move 语义,
D. 借用规则，
E. 如何把借用检查推迟到运行期?

## 2 实现宏`vec![]`

## 3 实现如下编程题

```rust
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// 请在在一个链表中每相邻的两个节点间插入一个结点, 插入节点的val为两个节点val的最大公约数
/// 例如： [10, 5, 6] -> [10, 5, 5, 1, 6]
///                          ^     ^
/// 复杂度要求： O(n)/O(1)
pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!()
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

fn main() {
    let n1 = ListNode::new(6);
    let mut n2 = ListNode::new(5);
    let mut n3 = ListNode::new(10);
    n2.next = Some(Box::new(n1));
    n3.next = Some(Box::new(n2));

    let new_head = insert_greatest_common_divisors(Some(Box::new(n3)));
    let mut p = &new_head;
    while let Some(node) = p {
        print!("{} ", node.val);
        p = &p.as_ref().unwrap().next;
    }
    println!("");
    // 10 5 5 1 6
}
```

## 4 利用Vec实现一个栈Stack(要求top, pop, push)，并给栈实现迭代器

## 5 实现快速排序算法的非递归版本
