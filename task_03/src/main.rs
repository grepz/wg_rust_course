#[derive(Debug)]
pub struct ListNode {
    pub val: Box<u8>,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: u8) -> Self {
        ListNode {
            next: None,
            val: Box::new(val),
        }
    }

    #[inline]
    pub fn new_boxed(val: Box<u8>) -> Self {
        ListNode { next: None, val }
    }

    #[inline]
    pub fn with_next(self, next: ListNode) -> Self {
        let next = Some(Box::new(next));
        Self { next, ..self }
    }

    #[inline]
    pub fn with_next_boxed(self, next: Box<ListNode>) -> Self {
        let next = Some(next);
        Self { next, ..self }
    }

    pub fn reverse(self) -> Self {
        let mut node  = Box::new(ListNode::new_boxed(self.val));
        let mut current = self.next;

        while let Some(mut item) = current {
            current = std::mem::replace(&mut item.next, Some(node));
            node = item;
        }

        *node
    }

    pub fn add_other(self, other: ListNode) -> ListNode {
        // Two numbers represented as lists where each digit is node
        // 123 as Head Some(3) -> Some(2) -> Some(1) -> None tail
        // you should return result of addition as list
        let mut l1 = Some(Box::new(self));
        let mut l2 = Some(Box::new(other));
        let mut carrier: u8 = 0;
        let mut result = None;
        loop {
            match (l1, l2) {
                (Some(l1_node), Some(l2_node)) => {
                    let sum = *l1_node.val + *l2_node.val + carrier;
                    if sum >= 10 {
                        carrier = 1;
                    } else {
                        carrier = 0;
                    }

                    let mut node = ListNode::new(sum % 10);
                    node.next = result;
                    result = Some(Box::new(node));

                    l1 = l1_node.next;
                    l2 = l2_node.next;
                }
                (None, Some(l2_node)) => {
                    let sum = *l2_node.val + carrier;
                    if sum >= 10 {
                        carrier = 1;
                    } else {
                        carrier = 0;
                    }

                    let mut node = ListNode::new(sum % 10);
                    node.next = result;
                    result = Some(Box::new(node));

                    l1 = None;
                    l2 = l2_node.next;
                }
                (Some(l1_node), None) => {
                    let sum = *l1_node.val + carrier;
                    if sum >= 10 {
                        carrier = 1;
                    } else {
                        carrier = 0;
                    }

                    let mut node = ListNode::new(sum % 10);
                    node.next = result;
                    result = Some(Box::new(node));

                    l1 = l1_node.next;
                    l2 = None;
                }
                (None, None) => {
                    if carrier != 0 {
                        let mut node = ListNode::new(carrier);
                        node.next = result;
                        result = Some(Box::new(node));
                    }
                    break;
                }
            }
        }
        result.unwrap().reverse()
    }
}

fn main() {
    println!("It's as simple as two times two!");
}

#[cfg(test)]
mod test_add {
    use super::*;

    #[test]
    fn zero() {
        let a = ListNode::new(0u8);
        let b = ListNode::new(0u8);

        let c = a.add_other(b);

        assert_eq!(0, *c.val);
        assert!(c.next.is_none());
    }

    #[test]
    fn simple_add() {
        // 123
        let a = ListNode::new(3u8).with_next(ListNode::new(2u8).with_next(ListNode::new(1u8)));

        // 456
        let b = ListNode::new(6u8).with_next(ListNode::new(5u8).with_next(ListNode::new(4u8)));

        let c = a.add_other(b);

        println!("======> simple_add: {:?}", c);

        let digit0 = c;
        assert_eq!(9u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(7u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(5u8, *digit2.val);
        assert!(digit2.next.is_none());
    }

    #[test]
    fn nine_in_the_middle() {
        // 1983
        let a = ListNode::new(3u8).with_next(
            ListNode::new(8u8).with_next(ListNode::new(9u8).with_next(ListNode::new(1u8))),
        );

        // 127
        let b = ListNode::new(7u8).with_next(ListNode::new(2u8).with_next(ListNode::new(1u8)));

        let c = a.add_other(b);

        println!("======> nine_in_the_middle: {:?}", c);

        let digit0 = c;
        assert_eq!(0u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(1u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(1u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(2u8, *digit3.val);
        assert!(digit3.next.is_none());
    }

    #[test]
    fn nine_in_the_middle_trans() {
        // 1983
        let a = ListNode::new(3u8).with_next(
            ListNode::new(8u8).with_next(ListNode::new(9u8).with_next(ListNode::new(1u8))),
        );

        // 127
        let b = ListNode::new(7u8).with_next(ListNode::new(2u8).with_next(ListNode::new(1u8)));

        let c = b.add_other(a);

        println!("======> nine_in_the_middle_trans: {:?}", c);

        let digit0 = c;
        assert_eq!(0u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(1u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(1u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(2u8, *digit3.val);
        assert!(digit3.next.is_none());
    }

    #[test]
    fn nines() {
        // 99 999
        let a = ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        ));

        // 9 999
        let b = ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        );

        let c = a.add_other(b);

        println!("======> nines: {:?}", c);

        let digit0 = c;
        assert_eq!(8u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(9u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(9u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(9u8, *digit3.val);
        assert!(digit3.next.is_some());

        let digit4 = digit3.next.unwrap();
        assert_eq!(0u8, *digit4.val);
        assert!(digit4.next.is_some());

        let digit5 = digit4.next.unwrap();
        assert_eq!(1u8, *digit5.val);
        assert!(digit5.next.is_none());
    }

    #[test]
    fn nines_trans() {
        // 99 999
        let a = ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        ));

        // 9 999
        let b = ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        );

        let c = b.add_other(a);

        println!("======> nines_trans: {:?}", c);

        let digit0 = c;
        assert_eq!(8u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(9u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(9u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(9u8, *digit3.val);
        assert!(digit3.next.is_some());

        let digit4 = digit3.next.unwrap();
        assert_eq!(0u8, *digit4.val);
        assert!(digit4.next.is_some());

        let digit5 = digit4.next.unwrap();
        assert_eq!(1u8, *digit5.val);
        assert!(digit5.next.is_none());
    }
}
