// Part 3.
// Lifetimes and move semantics.

// Problem 1.
// What went wrong? Copy strings properly.
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1.clone();
    assert_eq!(str1, str2);
}

// Problem 2.
// Question 2: How come it works fine here?
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Problem 3.
// These two don't work either. Fix by changing the type of "string" in the function
// copy_me ONLY, and by adjusting the parameter to "copy_me" where it's called.
#[test]
fn eat_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(&str1));
}

#[test]
fn eat_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1);
    assert_eq!(str1, str2);
}
//
fn copy_me(string: &String) -> String {
    string.clone()
}

// Problem 4.
// Can you implement this function?
// Add a lifetime specifier to the return type if you think it will help.
// If not, why not.
//fn new_ref_string<'a>(input_str: String) -> &'a String {
//    let copy_str = input_str.clone();
//    return &copy_str;
//}
//#[test]
//fn new_ref_string_tests() {
//    let input_str = String::from("mate");
//    assert_eq!(new_ref_string(*input_str), input_str);
//}
//
//// Problem 5.
//// Can you implement this function?
//// Add a lifetime specifier to the return type if you think it will help.
//// If not, why not.
//// fn new_ref_str() -> & str {
////     unimplemented!();
//// }
//
// Problem 6.
// Now we know how to implement this type of function. Implement it and write a test
// pick_longest_tests2() which passes all tests.
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.chars().count() >= s2.chars().count() {
        return &s1;
    } else {
        return &s2;
    }
}
#[test]
fn pick_longest2_tests() {
    let first = String::from("mate");
    let second = String::from("lol");
    assert_eq!(pick_longest2(&first, &second), &first);
}
//
// Problem 7.
// Write a function with a type signature which type checks the following test:
// and passes the test.
// This function compares it's second argument againsts all elements in it's first
// argument, returning those that are less than (<).
fn find_lesser_values<'a>(arr: &Vec<&'a str>, input_str: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for element in arr {
        if element.chars().count() < input_str.chars().count() {
            result.push(*element);
        }
    }
    return result;
}

#[test]
fn find_lesser_values_test() {
    assert_eq!(
        find_lesser_values(&vec!["foo", "bar", "foobar"], &"zzzzzzzz"),
        vec!["foo", "bar", "foobar"]
    );
    assert_eq!(
        find_lesser_values(&vec!["foo", "bar", "foobar"], &"bars"),
        vec!["foo", "bar"]
    );
    let result: Vec<&str> = Vec::new();
    assert_eq!(find_lesser_values(&vec!["foods"], &"food"), result);
    // Add more tests.
}
//
// Problem 8
// Move semantics present intersting API design choices not found in other languages.
// HashMap is an example of such a API.
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

// Specifically, the Entry API:
// https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

use std::collections::HashMap;

// Implement the following function which converts pairs from a vector,
// into key-value pairs in a hashmap.

fn vector_to_hashmap(v: &Vec<(i32, String)>) -> HashMap<i32, String> {
    let mut result = HashMap::new();
    for record in v {
        result.insert(record.0, record.1.clone());
    }
    return result;
}
#[test]
fn vector_to_hashmap_tests() {
    let mut result: Vec<(i32, String)> = Vec::new();
    result.push((1, String::from("mate")));
    let mut hash = HashMap::new();
    hash.insert(1, String::from("mate"));
    assert_eq!(vector_to_hashmap(&result), hash);
}
//
// Rust prevents us from deleting entries while iterating... Rewrite
// this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    h.retain(|k, _v| *k > 0);
}

#[test]
fn delete_negative_keys_tests() {
    let mut hash = HashMap::new();
    hash.insert(1, 2);
    hash.insert(3, 4);
    hash.insert(-1, 2);
    hash.insert(-2, 3);
    let mut result = HashMap::new();
    result.insert(1, 2);
    result.insert(3, 4);
    delete_negative_keys(&mut hash);
    assert_eq!(hash, result);
}

// For all entries in `add`: (k, v)
// If `k` exists in `merged`, append `v` to the value of `merged[k]`.
// If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.

// Use the Entry API:
// https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
// Use `or_insert` and `and_modify`.
fn merge_maps(merged: &mut HashMap<String, String>, add: HashMap<String,String>) {
    for (k,v) in add{
        merged.entry(k).or_insert(v);
    }
}

#[test]
fn merge_maps_tests() {
    let mut hash = HashMap::new();
    hash.insert(String::from("a"), String::from("b"));
    let mut add = HashMap::new();
    add.insert(String::from("c"), String::from("d"));
    let mut result = HashMap::new();
    result.insert(String::from("a"), String::from("b"));
    result.insert(String::from("c"), String::from("d"));
    merge_maps(&mut hash, add);
    assert_eq!(hash, result);
}
