//! The simple search API implemented in Rust by trying to directly translate the Python design.

/// Recursive implementation of linear search algorithm.
fn linear_search<T: PartialEq + PartialOrd>(
    item: T,
    item_list: &[T],
    is_sorted: bool,
) -> Option<usize> {
    if item_list.len() == 0 {
        None
    } else {
        if item == item_list[0] {
            Some(0)
        } else if is_sorted && item < item_list[0] {
            None
        } else {
            linear_search(item, &item_list[1..], is_sorted).map(|i| i + 1)
        }
    }
}

/// Recursive implementation of binary search algorithm.
fn binary_search<T: PartialEq + PartialOrd>(item: T, item_list: &[T]) -> bool {
    let l = item_list.len();

    if l == 0 {
        false
    } else {
        let mid = l / 2_usize;

        if item == item_list[mid] {
            true
        } else if item < item_list[mid] {
            binary_search(item, &item_list[..mid])
        } else {
            binary_search(item, &item_list[mid+1..])
        }
    }
}

pub enum SearchResult {
    Linear(Option<usize>),
    Binary(bool),
}

/// User-facing function to search for an item in a list.
pub fn search<T: PartialEq + PartialOrd + Ord>(
    item: T,
    item_list: &mut [T],
    check: bool,
    is_sorted: bool,
) -> SearchResult {
    if check {
        if !is_sorted {
            item_list.sort();
        }
        SearchResult::Binary(binary_search(item, item_list))
    } else {
        SearchResult::Linear(linear_search(item, item_list, is_sorted))
    }
}

fn main() {
    let mut item_list = [1, 2, 3, 4, 5];
    let search_result = match search(3, &mut item_list, true, true) {
        SearchResult::Binary(found) => found,
        SearchResult::Linear(_) => panic!("Linear search result returned!"),
    };
    println!("{}", search_result);
}
