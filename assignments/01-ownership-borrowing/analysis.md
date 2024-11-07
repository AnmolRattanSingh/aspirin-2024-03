# Analysis

## find_in_string

- `rustc` shows an error that `find_in_string` is `missing lifetime specifier`. I am not sure what lifetimes are yet so I do not know how to interpret the error. `rustc` does give a helpful description saying that it does not know if the return string is borrowed from the first string or the second string. The error makes sense but I am not sure how to fix it.

## doubly_linked_list

- `rustc` shows an error in `doubly_linked_list` when I try to write my push function. When adding a new element, I have a mutable reference to `self.head` when in the match statement. In the first `Some()` branch, I create a new Node and borrow self.head using `take()` to be the `next` field of the new Node. Then I set `head.next` to the new Node which edits `self.head`. But I am not allowed to do that since `self.head` is borrowed.
