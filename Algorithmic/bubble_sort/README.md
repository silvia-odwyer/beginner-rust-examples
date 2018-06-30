# Results
We all know bubble sort is slow to sort lists, but I thought it would be an easy and effective
benchmarking algorithm to test, since it's an algorithm we all know (or be easily learned).

Rust very narrowly beat Go by four seconds, showing that Go's ability to deal with huge data structures
is on par with Rust's.


## Sorting 100,000 list items with bubble sort

Rust -> 41.37 seconds

Go -> 45.09 seconds