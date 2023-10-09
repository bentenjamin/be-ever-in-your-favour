# be-ever-in-your-favour
Find int in list with odd count

### Example
```
input = "[[1,1,2,2,3,3,4],[9,9,8,8,7,7,6]]"

result = find_odd_counts(input)

print(result)
[4,6]
```

# Challenge readme
## Find Odds

### Problem

Given an array of integers, find the one that appears an odd number of times.

There will always be only one integer that appears an odd number of times.

Examples:

- [7] should return 7, because it occurs 1 time (which is odd).
- [0] should return 0, because it occurs 1 time (which is odd).
- [1,1,2] should return 2, because it occurs 1 time (which is odd).
- [0,1,0,1,0] should return 0, because it occurs 3 times (which is odd).
- [1,2,2,3,3,3,4,3,3,3,2,2,1] should return 4, because it appears 1 time (which is odd).

### Instructions

Design a program/script that can solve this problem in a fast/interesting/elegant way.
Your program should be able to accept a single command line argument for a filename.

This file will be a json formatted array of arrays of numbers ranging from 0-255.

Return your results as a single array.
For example If you ran a file with the 5 examples above the output would look like: [7,0,2,3,4]
