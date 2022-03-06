---
layout: post
title: Thinking in Terms of Types in Rust
date: 2022-03-06 21:58 +0530
---

{% include math.html %}

## Introduction

Let us begin by discussing a little bit about Rust: about what it brings to the table, when compared with other programming languages.
This will help set the stage for our discussion on types.

### Memory Safety

The key selling point of Rust is that it does away with an entire class of software bugs, known as _memory safety_ bugs.

Basically, languages like C and C++ offer precise control over memory, by means of references and pointers, and `malloc()` and `free()`.
This control, however, comes at the cost of inadvertently introducing a plethora of bugs into code:
  - _Double free:_ freeing the same memory twice or more times;
  - _Use after free:_ reading from memory that has already been freed; this is also known as a _dangling pointer_ situation;
  - _Null pointer dereference:_ dereferencing a pointer that does not point to any memory;
  - Reading from uninitialized memory;
  - Race conditions.

Rust manages to avoid all of these bugs by enforcing a compile-time _ownership_ and _borrowing_ system.
This system is defined by certain rules that center around:
  - the _owner_ of a piece of memory,
  - _borrowing_ a piece of memory for a limited period of time,
  - and _moving_ a piece of memory, i.e. transferring its ownership from one entity to another.

By default, the compiler ensures that the programmer adheres to all of these rules, throwing an error when any of them are violated, and in this manner the resulting program is free of memory safety bugs.

(Disclaimer: the use of `unsafe` allows the programmer to circumvent these rules for a limited block of code.
In this case, it is the programmer's responsibility to ensure that the code block at hand is memory safe.)

### Static Typing

At this point, let us move on to Rust's type system.
Rust, of course, is _statically typed_.

Let us contrast Rust's static typing system with a dynamic or _duck typing_ system, notably the one used by Python.
Consider the simple example of adding two variables.
A Python snippet to do the same goes like this:
```python
def add_two(a, b):
    return a + b

assert add_two(1, 2) == 3
```
If you try to add two variables that can't be added:
```python
class Foo:
    pass

a = Foo()
b = Foo()
add_two(a, b)
```
you get an error at runtime:
```
TypeError: unsupported operand type(s) for +: 'Foo' and 'Foo'
```
Duck typing is a system in which the sole constraint on the type of a variable is that it must, at runtime, have sufficient properties and behaviour required to make the current instruction valid.
In other words[^1]
> If it walks like a duck and quacks like a duck, then it must be a duck.

Rust's static type system is the opposite of this philosophy: it serves as a way of asserting the validity of each and every instruction in the program, at compile-time.
To see this, let us write the equivalent of the addition program in Rust.
On a first iteration, we might write the program like this:
```rust
fn add_two<T>(a: T, b: T) -> T {
    a + b
}

fn main() {
    assert_eq!(add_two(1, 2), 3);
}
```
The Rust compiler will give us an error:
```
error[E0369]: cannot add `T` to `T`
 --> add_two1.rs:3:7
  |
3 |     a + b
  |     - ^ - T
  |     |
  |     T
```
along with a suggestion that `T` should implement the `Add` trait of the standard library.
Let us try that:
```rust
fn add_two<T: std::ops::Add>(a: T, b: T) -> T {
    a + b
}
```
but we will still get an error:
```
error[E0308]: mismatched types
 --> add_two2.rs:3:5
  |
2 | fn add_two<T: std::ops::Add>(a: T, b: T) -> T {
  |            - this type parameter            - expected `T` because of return type
3 |     a + b
  |     ^^^^^ expected type parameter `T`, found associated type
  |
  = note: expected type parameter `T`
            found associated type `<T as Add>::Output`
```
along with a suggestion that we should restrict the `Add` bound on `T` by setting its `Output` type to `T`.
If we implement this suggestion it will compile.
```rust
fn add_two<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
```
You can see what the compiler and the type system have forced us to do.
First, they have ensured that the variables we are adding are of a type that can be added in the first place, and then ensured that the output of this addition is of the same type as its input.
Essentially, this leaves nearly no way for this addition operation to be invalid at runtime.

Rust's type system shares the philosophy of type systems in many functional programming (FP) languages (such as Haskell):[^2]
> Make illegal states unrepresentable.

[ Indeed, there are similarities between types in Rust and those in FP languages:
  - _traits_ in Rust are similar to Haskell's _typeclasses_,[^3]
  - enums in Rust are similar to _algebraic data types_ in F#, OCaml and Haskell,[^4]
  - and the _newtype pattern_ in Rust, i.e. when a struct is defined as a wrapper around a single type, such as the following:
    ```rust
    struct Id(usize);
    ```
    is a terminology borrowed from Haskell.[^5] ]

## The Moot Point

While the ownership and borrowing system takes care of program _correctness_ as far as memory safety is concerned, the type system takes the idea of correctness one level further, to ensure correctness of the entire program logic.

What I want to illustrate is that if we use Rust's type system as a way of simply silencing the compiler, then we are writing Rust code the wrong way.
The correct way is for Rust code to be **centered** around types.

Let us take a very simple example to explore this point.

## An Example

We take the example of a very simple search program, i.e. a program to search a list of items for a given item.

Two very common algorithms for this task are
  1. _linear_ search, in which we iterate over each element of the list, typically from left to right, checking whether the current element is equal to the given item;
  2. and _binary_ search, a more efficient algorithm which works by repeatedly dividing the list into halves.

Say we write functions for both of these algorithms.

We assume that a user wants to either:
  1. simply check whether the given item is present in the given list or not,
  2. or, find the index of the first occurrence of the given item in the given list.

In the first case, binary search is more appropriate, because it is much faster.
However, in the second case, we must use linear search, because binary search is not guaranteed to land at the first occurrence (it could land at any occurrence).

Another point to take into account is whether the list of items is sorted or not.
Binary search only works on a sorted list, so we should sort the list prior to running binary search if it is not already sorted.
While linear search does not require that the list be sorted, prior knowledge that the list is sorted can make it more efficient.

### Python Implementation

Consider the following design, described using a Python code snippet:
```python
def search(item, item_list, check=True, is_sorted=False):
    if check:
        if not is_sorted:
            item_list = sorted(item_list)
        return _binary_search(item, item_list)

    return _linear_search(item, item_list, is_sorted=is_sorted)
```
Essentially, we have a `search()` function that accepts
  - an `item`,
  - a list of items `item_list`,
  - an argument `check` that is set to `True` if we want to check whether the item is present in the list, and `False` if we want to find the index of the first occurrence,
  - and an argument `is_sorted` that is set to `True` if the list of items is already sorted, and `False` otherwise.

If we want to check whether the item is present in the list or not, we run binary search, sorting the list beforehand if it is not already sorted.
```python
def _binary_search(item, item_list):
    mid = len(item_list) // 2

    try:
        if item == item_list[mid]:
            return True
        elif item < item_list[mid]:
            return _binary_search(item, item_list[:mid])
        return _binary_search(item, item_list[mid+1:])
    except IndexError:
        return False
```
The binary search function returns `True` if the item was found and `False` if it was not.

Otherwise, if we want to find the index of the first occurrence, we run linear search, passing to the linear search function the `is_sorted` argument, leaving it to adjust depending on whether the list is sorted or not.
```python
def _linear_search(item, item_list, is_sorted=False):
    try:
        if item == item_list[0]:
            return 0
        elif is_sorted and item < item_list[0]:
            return None
        return _linear_search(item, item_list[1:], is_sorted=is_sorted) + 1
    except (IndexError, TypeError):
        return None
```
The linear search function returns the index of the first occurrence of the item, and `None` if the item was not found.

### Translation to Rust

Let us try to translate this design to Rust.
On a first attempt one might translate the binary search function to a function of the form
```rust
fn binary_search<T>(item: T, item_list: Vec<T>) -> bool { ... }
```
and the linear search function to
```rust
fn linear_search<T>(
    item: T,
    item_list: Vec<T>,
    is_sorted: bool,
) -> Option<usize> { ... }
```
After figuring out the ownership and borrowing, and adding required trait bounds, the signatures of these two functions slightly change:
```rust
fn binary_search<T: PartialEq + PartialOrd>(
    item: T,
    item_list: &[T],
) -> bool { ... }

fn linear_search<T: PartialEq + PartialOrd>(
    item: T,
    item_list: &[T],
    is_sorted: bool,
) -> Option<usize> { ... }
```
but that's about it.

Let us look at the `search()` function which wraps around these two functions.
If you think about what signature this function should have, you can immediately notice one problem: the return type.
While binary search returns a `bool`, linear search returns an `Option<usize>`, and two different return types are totally okay with a duck typing system, but in Rust the compiler will throw an error.

How do we get around this problem?
One way of looking at it is that the `search()` function can return two different types of results.
So, we can simply wrap these two different types inside an _enum_:
```rust
enum SearchResult {
    Linear(Option<usize>),
    Binary(bool),
}
```
and then we can write the `search()` function as follows:
```rust
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
```
(The `Ord` bound on `T` is required by the `.sort()` method.
Another point: `item_list` is accepted as a mutable reference `&mut [T]` because `.sort()` sorts the list in place; this is probably not the best thing to do because we do not want to change the original list and we will rectify this later.)

And this compiles. But is this good design?

Let us see one reason for why this is probably not good design.
Think a little bit about how this function will be used.
Consider one case in which we want to check if an item exists in a given sorted list:
```rust 
let search_result = match search(item, &mut item_list, true, true) {
    SearchResult::Binary(found) => found,
    SearchResult::Linear(_) => panic!("Linear search result returned!"),
};
```
Here, by passing the value of the `check` argument as `true`, we know that we want a binary search result, and not a linear search one.
However, the fact that `search()` returns an enum means that we have to check against both its `Binary` and `Linear` variants, even though we know that any sensible implementation would return only the `Binary` variant, and not the `Linear` one.

So, can we do better?

### A Type-Based Solution

As the name of this section indicates, let us try to explore a solution that attempts to identify, and implement, the core types involved in this problem.

One core entity involved in this problem is the list of items that has to be searched.
For this list to be searchable for a given item, its items must satisfy a fundamental property: namely, that we must be able to compare them for equality!

So consider the following definition:
```rust
type SearchList<T: PartialEq> = Vec<T>;
```
Additionally, we also want to make use of sorting the list / knowing that the list is sorted, hence we should also be able to _order_ items:
```rust
type SearchList<T: PartialEq + PartialOrd + Ord> = Vec<T>;
```
Another point is that it might be more appropriate to borrow the list of items rather than move ownership of the list into the `search()` function.
In that case we could use a reference to a slice, rather than a `Vec` ...
```rust
type SearchList<T: PartialEq + PartialOrd + Ord> = &[T];
```
... and this brings some lifetimes to take care of:
```rust
type SearchList<'a, T: PartialEq + PartialOrd + Ord> = &'a [T];
```
One piece of information that we would, ideally, like to store with the list, is whether it is sorted or not.
But as long as `SearchList` is a type alias, we can't do that.
Then let us change it to a struct:
```rust
struct SearchList<'a, T: PartialEq + PartialOrd + Ord> {
    list: &'a [T],
    is_sorted: bool,
}
```
We would also like to sort this list, if we need to.
For this, we could define an `impl` block on this `SearchList`, with a `sort()` method.
This `sort()` method could use the information of whether the list is already sorted or not, to avoid sorting twice.
In other words, we could do something like:
```rust
impl SearchList {
    fn sort(&self) {
        if !self.is_sorted {
            self.list.sort();
        }
    }
}
```
This won't compile.
We need to fix up some generics and lifetimes:
```rust
impl<T: PartialEq + PartialOrd + Ord> SearchList<'_, T> {
    fn sort(&self) {
        if !self.is_sorted {
            self.list.sort();
        }
    }
}
```
But it still will not compile.
And this time, the error we get is more significant:
```
|error 596| cannot borrow `*self.list` as mutable, as it is behind a `&` reference
||   |
|| 2 |     list: &'a [T],
||   |           ------- help: consider changing this to be mutable: `&'a mut [T]`
|| ...
|| 9 |             self.list.sort();
||   |             ^^^^^^^^^^^^^^^^ cannot borrow as mutable
```
To summarize, sorting of the list happens in-place and requires the list to be mutable.
We will live with this error for now; a little later we will show how we can fix this.
Note that the fix is not to declare the reference as mutable, because we don't want to change the original list!

Next, let us think a bit more on the result of a search operation.
Essentially, our aim is to convey two pieces of information:
  - whether the item was found, or not,
  - and optionally, find the index of the first occurrence of the item in the list.

Then consider the following design:
```rust
struct SearchResult {
    found: bool,
    index: Option<usize>,
}
```

The advantage of this is that we can have one, unified search result type, which can be returned by both the binary search algorithm and the linear search algorithm.
If binary search finds the element, it will return
```rust
SearchResult { found: true, index: None }
```
and if it doesn't
```rust
SearchResult { found: false, index: None }
```
If linear search finds the element, it will return
```rust
SearchResult { found: true, index: Some(...) }
```
and if it doesn't
```rust
SearchResult { found: false, index: None }
```
This easens up things on the user's side.
They know that if they have not asked for an index, and only want to know whether the item has been found or not, then they need simply check the value of the `found` field.
On the other hand, if they require an index, then they need to check the value of the `index` field.

On another note, if we were to come up with any other search algorithm, besides binary search and linear search, in the future, you can imagine that it would be easier to fit that algorithm into this framework, rather than coming up with another variant of the `SearchResult` enum as we had defined it before.

Then, the binary and linear search functions look something like this:
```rust
fn binary_search<T: PartialEq + PartialOrd + Ord>(
    search_list: SearchList<T>,
    item: T
) -> SearchResult { ... }

fn linear_search<T: PartialEq + PartialOrd + Ord>(
    search_list: SearchList<T>,
    item: T
) -> SearchResult { ... }
```
Note how we have repeated the trait bound
```rust
T: PartialEq + PartialOrd + Ord
```
four times now:
  1. in the definition of the `SearchList` struct,
  2. in the `impl` block of the `SearchList` struct,
  3. and in these two functions above.

It seems desirable to avoid this repetition.
We can do that by creating a trait that combines these bounds:
```rust
trait Searchable: PartialEq + PartialOrd + Ord {}
```
This is an empty trait, a _marker_ trait, which does not have any methods associated with it, but it makes sense because any searchable item satisfies these three properties!

This declaration says that anything that is `Searchable` must satisfy `PartialEq`, `PartialOrd` and `Ord`.
But, it does not imply the converse, i.e. that anything that satisfies `PartialEq`, `PartialOrd` and `Ord` will be `Searchable`.
To get this behaviour, we have to provide a _blanket_ implementation, which might go something like this:
```rust
impl<T> Searchable for T where T: PartialEq + PartialOrd + Ord {}
```

Overall, what we have done with this `Searchable` trait probably goes a little deeper into type semantics and might seem a bit overkill, but I think it's worth it.

### Need for a Cow

Now, it's easy enough to write the main `search()` function:
```rust
pub fn search<T: Searchable>(
    search_list: SearchList<T>,
    item: T,
    check: bool,
) -> SearchResult {
    if check {
        search_list.sort();
        binary_search(search_list, item)
    } else {
        linear_search(search_list, item)
    }
}
```
Then, the only problem remaining is the one with the `sort()` method on `SearchList`: remember that we got a compiler error requiring that the `list` field of `SearchList` should use a mutable reference, because the sorting happens in-place.

Let us think a bit about this.
Taking a mutable reference is not the solution, because ideally, we wouldn't want to modify the original list for a simple searching task.
But then, the only way to make sorting happen is to create a copy of the list.
But again, copying is expensive - we only want to copy the list when we are going to sort the list, and not when we are going to leave the list as it is.
In other words:
  - in the cases where we do not sort the list, we must maintain a reference to the list,
  - and in the case that we sort the list (the list is unsorted and we want to perform binary search), we must take a copy of the list.

How do we implement this in Rust?

The solution is a _smart pointer_ - the [clone-on-write (COW)](https://doc.rust-lang.org/std/borrow/enum.Cow.html#method.to_mut) smart pointer.

Essentially, the COW is an enum with two variants: `Owned` and `Borrowed`.
The `Borrowed` variant maintains a reference to some data, and whenever we require that data to be mutable, we can use the `to_mut()` method of the COW to clone the data and convert it into the `Owned` variant.

So now, we can use the COW like this:
```rust
struct SearchList<'a, T: Searchable>
{
    list: Cow<'a, [T]>,
    is_sorted: bool,
}
```
and then in the `sort()` method of `SearchList`, we can do:
```rust
impl<T: Searchable> SearchList<'_, T>
{
    fn sort(&mut self) {
        if !self.is_sorted {
            self.list.to_mut().sort();
        }
    }
}
```
And that's about it.
But we will still get a compiler error:
```
|error 277| the trait bound `[T]: ToOwned` is not satisfied in `Cow<'a, [T]>`
||   |
|| 9 |     list: Cow<'a, [T]>,
||   |           ^^^^^^^^^^^^ within `Cow<'a, [T]>`, the trait `ToOwned` is not implemented for `[T]`
||   |
||   = note: required because it appears within the type `Cow<'a, [T]>`
||   = note: only the last field of a struct may have a dynamically sized type
||   = help: change the field's type to have a statically known size
|| help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
||   |
|| 7 | struct SearchList<'a, T: Searchable> where [T]: ToOwned
||   |                                      ++++++++++++++++++
|| help: borrowed types always have a statically known size
```
Essentially, the compiler wants us to restrict `T` to only those types, such that a slice of this type can indeed be converted into an owned form.
(There may be slices of a type such that they cannot be owned, and only be borrowed.)

So, we have to add the following bound to `T`:
```rust
where [T]: ToOwned<Owned = Vec<T>>
```
i.e. allow only those types `T`, a slice of which can be converted into an owned form, and this owned form is none other than a `Vec<T>`.

Note that `T` already has a `Searchable` bound on it.
Along with `Searchable`, we now need this extra bound wherever `T` is used.

Should we include this bound as a part of the `Searchable` trait?
Including this bound as part of the `Searchable` trait will avoid repetition - remember how we defined the `Searchable` trait in the first place to avoid repetition of three other trait bounds.
But, intuitively, this bound doesn't really belong to the `Searchable` trait - whether a slice of a certain type can be converted to an owned form or not has nothing to do with the type being "searchable".
Hence, we will add this bound separately, along with the `Searchable` bound, wherever `T` is used, at the cost of some repetition.

### One Last Improvement

At this point, our Rust program will compile, but there is still one point we might be able to improve upon.

That point is the `check` argument to the `search()` function.
This argument is of type `bool`.
If it is `true`, then it means that we want to just check whether the given item is present in the list, and if it is `false`, then it means that we want to find the first occurrence of the item in the list.
So, if we wanted to do say, the former, we would call the `search()` function like so:
```rust
search(search_list, item, true)
```

Passing booleans to a Rust function?
That does not seem a very common practice, does it?

What's wrong about it?
Nothing's wrong, but it's just that it doesn't seem very readable.
In Python, which has a notion of optional arguments and default values, passing a boolean as a keyword argument is not unnatural at all:
```python
search(item_list, item, check=True)
```
But in Rust, which has no such notion, a boolean passed to a function can seem really odd.
What does that `true` mean?
What does it stand for?

A more 'idiomatic' way might be to replace this boolean by an enum; this will make the code more readable and clear.[^6]
Consider this enum for example:
```rust
pub enum SearchKind {
    CheckPresence,
    FindIndex,
}
```
The boolean argument `check` controls whether we are just checking for the presence of the item, or are trying to find the index.
This enum `SearchKind` has two variants, which respectively represent precisely just that!
Then, we can call the `search()` function like so:
```rust
search(search_list, item, SearchKind::CheckPresence)
```
I think you will agree that this is much better.

## Conclusion

That completes the solution to the searching problem!

Through this solution, we have played with various elements of Rust's type system, including structs, generics, traits and enums.
The part on the COW also shows how the ownership and borrowing system can also influence types used in the program.

The main essence of our discussion is that good Rust code requires a close understanding of the core entities and types involved in the program.
Working out these entities and types may end up being quite involved, and as a result it may take more time to express our ideas and get them to execute, but overall this can lead to extremely elegant -- and correct -- code.

## Code

The code discussed in this post can be found [here](https://github.com/codeandfire/codeandfire.github.io/tree/main/code/2022-03-06-thinking).

## Note

It was not until I finished writing all of this that I realized that binary search is $\mathcal{O}(\log n)$, and an efficient sort algorithm like quicksort is $\mathcal{O}(n \log n)$, which means that a combination of sorting beforehand followed by binary search is $\mathcal{O}(n \log n)$, and this is more expensive than doing plain linear search which is $\mathcal{O}(n)$.
In other words, if the list is unsorted, it is more efficient to perform linear search, rather than binary search after sorting.

But really, the bit about sorting was just to add a little more complexity to the problem, so that we would get more opportunities to see how we can work with Rust's type system.
I hope you can see why I framed the problem this way, and ignore that little technicality about linear search being better suited for an unsorted list.

---

<br>

[^1]: <https://en.wikipedia.org/wiki/Duck_typing>
[^2]: <https://kowainik.github.io/posts/haskell-mini-patterns#make-illegal-states-unrepresentable>
[^3]: See answers to this StackOverflow question: <https://stackoverflow.com/q/28123453>
[^4]: <https://doc.rust-lang.org/book/ch06-00-enums.html>
[^5]: <https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types>
[^6]: Writing idiomatic libraries in Rust, by Pascal Hertleif. <https://youtu.be/0zOg8_B71gE?t=725>
