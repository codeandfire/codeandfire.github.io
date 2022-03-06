"""A simple search API in Python."""

def _linear_search(item, item_list, is_sorted=False):
    """Recursive implementation of linear search algorithm.

    Returns the index of the first occurrence of the item in the list.
    Returns None if the item was not found.
    """

    try:
        if item == item_list[0]:
            return 0
        elif is_sorted and item < item_list[0]:
            return None
        return _linear_search(item, item_list[1:], is_sorted=is_sorted) + 1
    except (IndexError, TypeError):
        return None

def _binary_search(item, item_list):
    """Recursive implementation of binary search algorithm.

    Returns True if the item was found in the list and False if it was not.
    """

    mid = len(item_list) // 2

    try:
        if item == item_list[mid]:
            return True
        elif item < item_list[mid]:
            return _binary_search(item, item_list[:mid])
        return _binary_search(item, item_list[mid+1:])
    except IndexError:
        return False

def search(item, item_list, check=True, is_sorted=False):
    """Search for an item in a list.

    Arguments:
        item : item to search for
        item_list : list of items in which the given item must be searched
                    for
        check :
            If True, return True/False depending upon whether the given
            item is present in the list or not.
            If False, return the index of the first occurrence of the item
            in the list; return None if the item is not present in the
            list.
        is_sorted :
            specify True/False to indicate whether the given list is sorted
            or not.
    """

    if check:
        if not is_sorted:
            item_list = sorted(item_list)

        return _binary_search(item, item_list)

    return _linear_search(item, item_list, is_sorted=is_sorted)


# some tests to verify our implementations
assert _linear_search(5, list(range(10))) == 5
assert _linear_search(0, list(range(10))) == 0
assert _linear_search(9, list(range(10))) == 9
assert _linear_search(15, list(range(10))) is None

assert _binary_search(5, list(range(10)))
assert _binary_search(0, list(range(10)))
assert _binary_search(9, list(range(10)))
assert not _binary_search(15, list(range(10)))

assert search(5, list(range(10)), check=True, is_sorted=True)
assert not search(15, list(range(9, -1, -1)), check=True, is_sorted=False)
assert search(5, list(range(10)), check=False, is_sorted=True) == 5
assert search(15, list(range(9, -1, -1)), check=False, is_sorted=False) is None
