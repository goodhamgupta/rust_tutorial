use std::fs::File;
use std::io::Read;

// Part 2.
// Make the following failing functions/tests pass.
// Answer the questions as a comment next to the problems.

// Problem 1.
// Create functions split_ref and split_clone such that
// all the following tests will pass. Feel free to use Rust's split method
// (https://doc.rust-lang.org/std/primitive.slice.html#method.split)
// as needed.

// split_ref must have the return type Vec<&str>
// split_clone must have the return type Vec<String>

fn split_ref(v: &str) -> Vec<&str> {
    let response = v.split_whitespace().collect::<Vec<&str>>();
    response
}
#[test]
fn split_ref_tests() {
    let string = "Hello World!".to_string();
    assert_eq!(split_ref(&string), ["Hello", "World!"]);
    assert_eq!(split_ref("Hello World!"), &["Hello", "World!"]);
    assert_eq!(split_ref("Hello World!"), vec!["Hello", "World!"]);
}

fn split_clone(v: &str) -> Vec<&str> {
    let clone = v.clone();
    let response = clone.split_whitespace().collect::<Vec<&str>>();
    response
}

#[test]
fn split_clone_tests() {
    let string = "Hello World!".to_string();
    assert_eq!(split_clone(&string), ["Hello", "World!"]);
    assert_eq!(split_clone("Hello World!"), &["Hello", "World!"]);
    assert_eq!(split_clone("Hello World!"), vec!["Hello", "World!"]);
}

// Problem 2.
// Write function pick_longest which picks the longests of two string-ish
// objects. Please make the function as general as possible (i.e do not
// take "String" as a parameter).
//
// From simplicity return a new String, later we will learn how to return
// references. Write additional tests.
//
fn pick_longest<T: ToString>(first: T, second: T) -> String {
    let first_str = first.to_string();
    let second_str = second.to_string();
    if first_str.len() >= second_str.len() {
        return String::from(first_str);
    } else {
        return String::from(second_str);
    }
}

#[test]
fn pick_longest_tests() {
    assert_eq!(pick_longest(&"cat".to_string(), &"dog".to_string()), "cat");
}

#[test]
fn pick_longest_numbers_tests() {
    assert_eq!(
        pick_longest(&"1234".to_string(), &"123".to_string()),
        "1234"
    );
}

// Question 1:
//For the curious, attempt to return reference, that is:
//
// fn pick_longest(???) -> &str
//
// What goes wrong when you try to implement this function? Why is this
// the case?
// For a function to return a variable with reference, it must include the lifetime for the
// reference. Since the function does not take any parameter by reference, lifetime elision does
// not occur automatically.

// Problem 3.
// Write a function that returns all the contents of a file as a single String.

// DO NOT USE the assocated function std::fs::read_to_string

// Instead use File::open, and the method read_to_string
// (https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string)

//// Use .expect("ignoring error: ") to ignore the Result<...> type in open() and
//// read_to_string. We learn error handling later.
fn print_contents_of_file(path: &str) -> String {
    let mut f = File::open(path).expect("ignoring error: ");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer);
    return buffer.trim().to_string();
}

#[test]
fn print_contents_of_file_tests() {
    let response =
        print_contents_of_file(&"/home/goodhamgupta/shubham/rust_tutorial/homework1/src/test.txt");
    assert_eq!(response, "dummy");
}

//// Problem 4.
//// Why does the following implementation not work as expected?
//// Fix by changing the type signature of add1 and the way it's called on add1_test().
//// do NOT change the return type.
//

fn add1(x : &mut i32) -> () {
    *x += 1;
}

#[test]
fn add1_test() {
    let mut x = 1;
    add1(&mut x);
    assert_eq!(x, 2);
}
//
//// Problem 5.
//// Error says: cannot assign to immutable borrowed content `*str1`
//// But we declared it mutable? Fix by changing only the line below.
fn mut2() {
    let hello = String::from("hello");

    // CHANGE ONLY THIS LINE:
    let str1: &mut String = &mut String::from("str1");

    *str1 = hello;
}
