# Abstract Algebra
An abstract algebra cli written in Rust.

## Listing Group Elements
To list the elements in a group, use `abstract-algebra group <group_id> --elements`.
```
$ abstract-algebra group S3 --elements
Elements of S3:
[order 6] {(0, 1, 2), (0, 2, 1), (2, 0, 1), (2, 1, 0), (1, 2, 0), (1, 0, 2)}
```

## Conjugacy classes
Use `abstract-algebra group <group_id> --conjugacy-classes` to list the conjugacy classes of a group.
```
$ abstract-algebra group A4 --conjugacy-classes
Conjugacy classes of A4:
[1 element] {(0, 1, 2, 3)}
[4 elements] {(0, 3, 1, 2), (1, 2, 0, 3), (3, 0, 2, 1), (2, 1, 3, 0)}
[4 elements] {(0, 2, 3, 1), (2, 0, 1, 3), (1, 3, 2, 0), (3, 1, 0, 2)}
[3 elements] {(2, 3, 0, 1), (1, 0, 3, 2), (3, 2, 1, 0)}
```
You can also use `abstract-algebra group <group_id> --center` to list the members of trivial conjugacy classes.

## Supported Groups
* Alternating groups `A<n>`
* Symmetric groups `S<n>`
* Cyclic groups `Z<n>`
* Dihedral groups of order 2n `D<n>`
* Quaternion group `Q8`

## Upcoming Features
* Direct product of groups
* Formatting permutations with cycle notation