// replace value inside option if it is some to b: Box<i64>
// you can't use pattern matching
pub fn replace_value_inside_option(op: Option<i32>, b: Box<i64>) -> Option<Box<i64>> {
    if op.is_some() {
        Some(b)
    } else {
        None
    }
}

// function should return count of items in collection c
pub fn count_items_in_collection_stupid(c: impl Iterator<Item = i32>) -> usize {
    let mut counter: usize = 0;
    for _ in c {
        counter += 1;
    }
    counter
}


// function should return count of items in collection c
pub fn count_items_in_collection(c: impl Iterator<Item = i32>) -> usize {
    c.count()
}


// change all true item to false
// return Vec<bool> and count of changed items
pub fn change_true_to_false(c: impl Iterator<Item = bool>) -> (Vec<bool>, usize) {
    let vec_proc_f =
        |(mut v_acc, sz): (Vec<bool>, usize), val| -> (Vec<bool>, usize) {
            v_acc.push(false);
            (v_acc, match val {true => sz + 1, false => sz})
        };
    c.fold(
        (Vec::new(), 0),
        vec_proc_f
    )
}

// Return all even items starts from first 2 and ends first 32
// example
// input:
// 1,8,4,2,5,7,8,16,18,32,2,1,64
// output
//       2,    8,16,18 <-- without last 32
pub fn composite(c: impl Iterator<Item = i32>) -> Vec<i32> {
    c.skip_while(|item| *item != 2)
        .take_while(|item| *item != 32)
        .filter(|item| 0 == item % 2)
        .collect()
}

pub fn composite_stupid(c: impl Iterator<Item = i32>) -> Vec<i32> {
    // Should be probably done with skip_while/take_while
    let mut start = false;
    let mut end = true;
    // Use drain_filter?
    c.filter(|item| -> bool {
        if *item == 2 {
            start = true;
        } else if *item == 32 {
            end = false;
        }
        start && end && 0 == *item % 2
    }).collect()
}

// with *
// Return all even items starts from first 2 and ends first 32
// example
// input:
// 1,8,4,2,5,7,8,16,18,32,2,1,64
// output
//       2,    8,16,18,32 <-- with last 32
pub fn composite2(c: impl Iterator<Item = i32>) -> Vec<i32> {
    let mut ends_met = false;

    c.skip_while(|item| *item != 2)
        .take_while(|item| -> bool {
            let cond = !ends_met;
            if *item == 32 {
                ends_met = true;
            }
            cond
        })
        .filter(|item| 0 == *item % 2)
        .collect()
}


// combine string from collection into semi-colon separated string
// one, two, three
// into
// one;two;three;
pub fn words(c: impl Iterator<Item = String>) -> String {
    let mut str = String::new();
    for x in c {
        str.push_str(&x);
        str.push(';');
    }
    str
}

fn main() {
    println!("Easy");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn replace_value_inside_option_none() {
        assert!(replace_value_inside_option(None, Box::new(1)).is_none());
    }

    #[test]
    fn replace_value_inside_option_some() {
        let b = Box::new(1);
        let b_address: *const i64 = std::ops::Deref::deref(&b);

        let r = replace_value_inside_option(Some(1), b);

        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(*r, 1);
        assert_eq!(b_address, std::ops::Deref::deref(&r) as *const i64);
    }

    #[test]
    fn test_count_items_in_collection() {
        assert_eq!(
            count_items_in_collection(Option::<i32>::None.into_iter()),
            0usize
        );
        assert_eq!(count_items_in_collection(Some(1).into_iter()), 1usize);
        assert_eq!(
            count_items_in_collection(vec![1, 2, 3, 4, 5].into_iter()),
            5usize
        );
    }

    #[test]
    fn test_change_true_to_false() {
        let (c, s) = change_true_to_false(Vec::<bool>::new().into_iter());
        assert_eq!(c.len(), 0);
        assert_eq!(s, 0usize);

        let (c, s) = change_true_to_false(vec![false, false, false].into_iter());
        assert_eq!(c, vec![false, false, false]);
        assert_eq!(s, 0usize);

        let (c, s) = change_true_to_false(vec![true, false, false, false, true, true].into_iter());
        assert_eq!(c, vec![false, false, false, false, false, false]);
        assert_eq!(s, 3usize);
    }

    #[test]
    fn test_composite() {
        let r = composite(vec![1, 8, 4, 2, 5, 7, 8, 16, 18, 32, 2, 1, 64].into_iter());
        assert_eq!(r, vec![2, 8, 16, 18])
    }

    #[test]
    fn test_composite2() {
        let r = composite2(vec![1, 8, 4, 2, 5, 7, 8, 16, 18, 32, 2, 1, 64].into_iter());
        assert_eq!(r, vec![2, 8, 16, 18, 32])
    }

    #[test]
    fn test_words() {
        assert_eq!(words(vec![].into_iter()), String::from(""));
        assert_eq!(
            words(vec![String::from("one")].into_iter()),
            String::from("one;")
        );
        assert_eq!(
            words(vec![String::from("one"), String::from("two")].into_iter()),
            String::from("one;two;")
        );
    }
}
