//! The simple search API implemented in Rust with a focus on types.

use std::borrow::Cow;

pub trait Searchable: PartialEq + PartialOrd + Ord {}
impl<T> Searchable for T where T: PartialEq + PartialOrd + Ord {}

#[derive(Clone, Debug)]
pub struct SearchList<'a, T: Searchable>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    list: Cow<'a, [T]>,
    is_sorted: bool,
}

impl<T: Searchable> SearchList<'_, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn sort(&mut self) {
        if !self.is_sorted {
            self.list.to_mut().sort();
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult {
    found: bool,
    index: Option<usize>,
}

/// Recursive implementation of binary search algorithm.
fn binary_search<T: Searchable>(search_list: SearchList<T>, item: T) -> SearchResult
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    let l = search_list.list.len();

    if l == 0 {
        SearchResult {
            found: false,
            index: None,
        }
    } else {
        let mid = l / 2_usize;

        if item == search_list.list[mid] {
            SearchResult {
                found: true,
                index: None,
            }
        } else if item < search_list.list[mid] {
            binary_search(
                SearchList {
                    list: Cow::Borrowed(&search_list.list[..mid]),
                    ..search_list
                },
                item,
            )
        } else {
            binary_search(
                SearchList {
                    list: Cow::Borrowed(&search_list.list[mid + 1..]),
                    ..search_list
                },
                item,
            )
        }
    }
}

/// Recursive implementation of linear search algorithm.
fn linear_search<T: Searchable>(search_list: SearchList<T>, item: T) -> SearchResult
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    if search_list.list.len() == 0 {
        SearchResult {
            found: false,
            index: None,
        }
    } else {
        if item == search_list.list[0] {
            SearchResult {
                found: true,
                index: Some(0),
            }
        } else if search_list.is_sorted && item < search_list.list[0] {
            SearchResult {
                found: false,
                index: None,
            }
        } else {
            let search_result = linear_search(
                SearchList {
                    list: Cow::Borrowed(&search_list.list[1..]),
                    ..search_list
                },
                item,
            );

            SearchResult {
                index: search_result.index.map(|i| i + 1),
                ..search_result
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum SearchKind {
    CheckPresence,
    FindIndex,
}

/// User-facing function to search for an item in a list.
pub fn search<T: Searchable>(
    mut search_list: SearchList<T>,
    item: T,
    kind: SearchKind,
) -> SearchResult
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    match kind {
        SearchKind::CheckPresence => {
            search_list.sort();
            binary_search(search_list, item)
        }
        SearchKind::FindIndex => linear_search(search_list, item),
    }
}

#[cfg(test)]
mod tests {
    //! Some tests to verify our implementations.

    use super::*;

    #[test]
    fn test_linear_search() {
        let search_list = SearchList {
            list: Cow::Borrowed(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            is_sorted: true,
        };

        assert!(linear_search(search_list.clone(), 5).found);
        assert_eq!(linear_search(search_list.clone(), 5).index, Some(5));
        assert!(linear_search(search_list.clone(), 0).found);
        assert_eq!(linear_search(search_list.clone(), 0).index, Some(0));
        assert!(linear_search(search_list.clone(), 9).found);
        assert_eq!(linear_search(search_list.clone(), 9).index, Some(9));

        assert!(!linear_search(search_list.clone(), 15).found);
        assert_eq!(linear_search(search_list.clone(), 15).index, None);
    }

    #[test]
    fn test_binary_search() {
        let search_list = SearchList {
            list: Cow::Borrowed(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            is_sorted: true,
        };

        assert!(binary_search(search_list.clone(), 5).found);
        assert_eq!(binary_search(search_list.clone(), 5).index, None);
        assert!(binary_search(search_list.clone(), 0).found);
        assert_eq!(binary_search(search_list.clone(), 0).index, None);
        assert!(binary_search(search_list.clone(), 9).found);
        assert_eq!(binary_search(search_list.clone(), 9).index, None);

        assert!(!binary_search(search_list.clone(), 15).found);
        assert_eq!(binary_search(search_list.clone(), 15).index, None);
    }

    #[test]
    fn test_search() {
        let search_list_sorted = SearchList {
            list: Cow::Borrowed(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            is_sorted: true,
        };
        let search_list_unsorted = SearchList {
            list: Cow::Borrowed(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
            is_sorted: false,
        };

        assert!(search(search_list_sorted.clone(), 5, SearchKind::CheckPresence).found);
        assert!(!search(search_list_unsorted.clone(), 15, SearchKind::CheckPresence).found);
        assert_eq!(
            search(search_list_sorted.clone(), 5, SearchKind::FindIndex).index,
            Some(5)
        );
        assert_eq!(
            search(search_list_unsorted.clone(), 15, SearchKind::FindIndex).index,
            None
        );
    }
}
