/*
---Find Non-Consecutive Numbers---
Your task is to find all the elements of an array that are non consecutive.

A number is non consecutive if it is not exactly one larger than the previous element in the array.
The first element gets a pass and is never considered non consecutive.

Create a function name all_non_consecutive

E.g., if we have an array [1,2,3,4,6,7,8,15,16] then 6 and 15 are non-consecutive.

You should return the results as an array of tuples with two values: the index of the
non-consecutive number and the non-consecutive number.

E.g., for the above array the result should be:

[
  ( 4, 6 ),
  ( 7, 15 )
]
If the whole array is consecutive or has one element then return an empty array.

The array elements will all be numbers. The numbers will also all be unique and in ascending order.
The numbers could be positive and/or negetive and the gap could be larger than one.
 */

fn main() {
    println!("{:?}", all_non_consecutive(&[1, 2, 3, 4, 6, 7, 8, 10]));
    println!("{:?}", all_non_consecutive(&[]));
    println!("{:?}", all_non_consecutive(&[1]));
}

fn all_non_consecutive(arr:&[i32]) -> Vec<(usize, i32)> {
    if arr.len() <= 1 {
        return Vec::new();
    }

    let mut return_vec:Vec<(usize, i32)> = Vec::new();
    let mut previous_value:i32 = arr[0];

    for (index, val) in arr.iter().enumerate() {
        if index == 0 {
            continue;
        }

        if previous_value + 1 != *val {
            return_vec.push((index, *val));
        }

        previous_value = *val;
    }

    return_vec
}
