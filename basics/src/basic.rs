
#[test]
fn basics() {
    let immutable = 5;
    let mut mutable = 6;
    mutable = 2;

    let spaces = "     "; // auto type deduction
    let spaces = spaces.len(); // "shadowing" new variable with same name

    const CONSTANT: u32 = 100_000; // constant (type must be specified)

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (t0, t1, t2) = tuple;
    assert_eq!(t0, tuple.0);
    assert_eq!(t1, tuple.1);
    assert_eq!(t2, tuple.2);

    let array: [i32; 5] = [3, 3, 3, 3, 3];
    assert_eq!([3; 5], array);
    let x = array[2]; // runtime error if invalid array acces (only in debug mode)

    if 1 == 1 { // if statment
         // always has body with curly brackets
    }

    let if_expression = if 5 == 3 { 5 } else { 6 };

    let loop_result = loop {
        println!("infinite loop");
        break 5;
    };
    assert_eq!(loop_result, 5);

    for element in array.iter() {
        println!("the value is: {}", element);
    }

    // countdown
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn expression_return() -> i32 {
    69 * 420
}

#[test]
fn ownership_and_borrowing() {
    let primitive = 423; // lives on stack
    let string_literal: &'static str = "hello"; // reference to memory in binary
    let string = String::from("hello"); // lives on heap (~ smart pointer)

    {
        let temp = 5;
        let also_temp = String::from("ey");
    } // variable leaves scope -> gets "dropped"

    let original = 5;
    let copy = original; // is copied, because primitive

    let original = String::from("I like to move it, move it");
    let move_destination = original; // moved in new memory; `original` becomes invalid
    let copy = original.clone(); // copied, because has `copy` trait

    let value = 5;
    let reference = &value;
    assert_eq!(value, *reference);
    let other_value = 10;

    let mutable_reference = &mut other_value;
    *mutable_reference = 11;
    assert_eq!(value, *mutable_reference);

    // borrowing rules
    let mut original = String::new();
    let mutable_reference = &mut original;
    let reference = &original; // can't borrow twice at a time

    let mut original = String::new();
    let reference1 = &original;
    let reference2 = &original;

    // slices
    let string = String::from("Hello World");
    let string_slice = &string[0..5]; // must be continuous part of memory
    assert_eq!("Hello", string_slice);

    let array = [1, 2, 3, 4, 5];
    let array_slice = &array[1..3];
    assert_eq!([2, 3, 4], array_slice);

    // passing function arguments
    let string = String::new();
    take_reference(&string);
    take_mut_ref(&mut string);
    string = take_and_give_back_ownership(string);
    take_ownership(string);
}

// function arguments
fn take_ownership(arg: String) {
    println!("Taken ownership of `{}`", arg);
}

fn take_and_give_back_ownership(mut arg: String) -> String {
    arg.push_str("shesh");
    arg
}

fn take_reference(arg: &String) {
    println!("Length of string is {}", arg.len());
}

fn take_mut_ref(arg: &mut String) {
    arg.push_str("soos");
}

fn first_word_pos(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str {
    // str is the string-slice type
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// structs

#[derive(Debug)] // for debug printing
struct User {
    username: String, // field
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn code_holder_3() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 69,
    };

    user1.email = String::from("diffrentemail@gmail.com");

    let user2 = User {
        email: String::from("lel@mail.com"),
        username: String::from("diffrentName"),
        ..user1 // copies the other values of user1
    };

    // println!("user is {}", user2); error because User doesn't impelement 'std::fmt::Display'
    println!("user is {:?}", user2); // using output format 'Debug'
}

// tuple struct
struct Color(i32, i32, i32); // is its own type
struct Point(i32, i32, i32);

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    // method (because takes self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function (bacause doesn't take self) -> Rect::square()
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

// Enums

enum IpAddrKind {
    // is a custom data type
    V4, // variant of enum
    V6,
}

struct IpAddrBad {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr {
    // better way, also diffrent data types possible
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {}

fn code_holder_4() {
    let four = IpAddrKind::V4; // are of same type
    let six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },    // struct
    Write(String),              // tuple struct
    ChangeColor(i32, i32, i32), // tuple struct
}

impl Message {
    fn call(&self) {
        // code
    }
}

// option

enum CustomOption<T> {
    // replaces 'null'-value
    Some(T),
    None,
}

fn code_block_5() {
    let some_number = Some(5); // option
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}

// match: control flow operator

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn matches_are_exhaustive(val: u8) {
    match val {
        1 => println!("one"),
        2 => println!("two"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

// if let

fn if_let() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // equivalent to
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

// collections

fn code_holder_6() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);

    let v = vec![1, 2, 3, 4, 5];
    // two ways to access vector
    let third: &i32 = &v[2]; // panics if fails
    match v.get(2) {
        // doesn't panic
        Some(third) => (),
        None => (),
    }

    // iterating
    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 50;
    }

    // multiple type vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

// strings

// str is implemented in the core language and String is in the standard library

fn code_holder_7() {
    let mut s = String::new();
    let data = "inital contents"; // implements 'Display' trait
    let mut s = data.to_string();
    s.push_str("bar");
    s.push('a');
    let s1 = String::from("Hello ");
    let s2 = String::from("World");
    let s3 = s1 + &s2; // s1 was moved! (fn add(self, s: &str) -> String)

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    // you can't index into string, because of ambigueties and other reasons -> be more percise

    // slices... not so appropriate
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 4 bytes -> "Зд"

    // best way: chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}

// Hash Maps

fn code_holder_8() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // morphing collections
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let inital_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(inital_scores.iter()).collect();
}

// errors

fn code_holder_9() {
    // panicing!
    // If rust panics before it quite it's starts unwinding  (stack is cleared up), which takes a lot of time -> alternative abort (in Cargo.toml: panic = 'abort')
    panic!("crash and burn");

    // Result
    use std::fs::File;
    use std::io::ErrorKind;
    use std::io::Read;
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let f = File::open("hello.txt").unwrap(); // returns value if okay, panics otherwise
    let f = File::open("hello.txt").expect_err("Own error message"); // same as unwrap() just with custom error message

    // propagating error
    fn read_username_from_file_verbose() -> Result<String, std::io::Error> {
        // verbose way
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_username_from_file() -> Result<String, std::io::Error> {
        // better way with ? operator
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?; // if ok expression has value, if Err then function returns with error
        Ok(s)
    }
}

// generics (similar to C++ typenames/templates)

enum own_Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point1<T> {
    x: T,
    y: T,
}

impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// traits

trait Summarizable {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summarizable for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.summarize_author(),
            self.location
        )
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summarizable for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

// traits as parameters/ Trait bounds

fn notify(item: impl Summarizable) {
    println!("Breaking news! {}", item.summarize());
}
// ^ syntactic sugar for:

// fn notify<T: Summarizable>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }

fn notfiy<T: Summarizable + std::fmt::Display>(item1: T) {}

// when many traits are used -> prefer 'where'-clauses to not clutter the funciton definition

fn some_function<T, U>(t: T, u: U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    4
}

fn returns_summarizable() -> impl Summarizable {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of cource, as you probablay already know people"),
        reply: false,
        retweet: false,
    }
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Trait bounds to conditionally implement Methods

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// conditional implementation (only if traits are Display + PartialOrd)
impl<T: std::fmt::Display + std::cmp::PartialOrd> Pair<T> {
    fn cmp_disply(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// implement a trait if the type implements another train --- alias blanket implementations

// impl<T: std::fmt::Display> ToString for T { // if T already implements Display, than it also implements ToString
// }

// lifetimes

// lifetimes gurantee, that references are still valid, when used.
// Most of the time they are implicitly inferred. If they can't, they have to be explicitly specified

// &i32; a reference
// &'a i32; a reference with the explicit lifetime "'a"
// &'a mut i32; a mutable reference with the explicit lifetime "'a"

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // now the compiler knows, how long the return value can live. (as long as the smaller lifetime of x or y)
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str, // if struct holds reference, a explicit lifetime is required
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// static lifetimes (references live for entire duration of program)... applies to all string ltierals
fn code_holder_10() {
    let s: &'static str = "I have a static lifetime.";
}

// all generics together

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// closures

fn code_holder_11() {
    // types are automatically inferred (but can be explicitly specified)
    let some_closure = |arg| {
        println!("this is the argument: {}", arg);
    };

    let minimalist_closure = |x| x; // returns itself

    some_closure(5);
    minimalist_closure("lel");

    // pattern: memorization / lazy evaluation

    struct NoArgsCacher<T>
    where
        T: Fn() -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> NoArgsCacher<T>
    where
        T: Fn() -> u32,
    {
        fn new(calculation: T) -> NoArgsCacher<T> {
            NoArgsCacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)();
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    use std::thread;
    use std::time::Duration;

    let mut expensive_result = NoArgsCacher::new(|| {
        println!("performing expensive calculation...");
        thread::sleep(Duration::from_secs(2));
        420
    });

    // TODO: create better Cacher with generics and a hash-table (args-result pairs)
}

// iterators

// zero-cost abstraction -> are very fast USE THEM!

fn code_holder_12() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    pub trait CustomIteratorTrait {
        type Item; // associated type

        fn next(&mut self) -> Option<Self::Item>;
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum(); // iter has been consumed (moved) -> cannot be moved any more
    }

    #[test]
    fn iterator_map() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // collect() must be called because iterators are lazy
        assert_eq!(v2, vec![2, 3, 4]);
    }

    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    // own iterator
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}

// cargo and creates
// //! Another documentation style, which is at the top of the page, generally in the crate root
// //! re-exports are listed in documentation -> expose them
/// Documentation comment (3 slashes)
/// will be used to generate HTML documentation (cargo doc --open) -> runs rustdoc
/// supports MarkDown!
/// Some commonly used headings
/// # Examples
/// # Panics
/// # Errors (when it returns Result)
/// # Safety (if unsafe to call)
/// '''
/// assert_eq!(true, true);
/// '''
/// this code example will be run as a test with (cargo test)!!! AWESOME
fn documented_function() {}

// smart pointers

// Vec and String are smart pointers, because they point at data and have some additional metadata
// allocate data on heap

// Box<T> for storing data on heap (no performnace overhead)
// usages: dynamic memory (unknown size), transfer ownership without copying, value that implements a trait but the type doesn't matter

fn code_holder_13() {
    let b = Box::new(5);

    // recursive types and Cons List

    // enum List { idea
    //     Cons(i32, List),
    //     Nil,
    // }
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::Cons;
    use List::Nil;

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // dereferencing
    let x = 5;
    let y = &x;
    // assert_eq!(5, x); doesn't compile because diffrent types
    assert_eq!(5, *y); // dereferenced

    // deref trait
    struct FakeBox<T>(T);

    impl<T> FakeBox<T> {
        fn new(x: T) -> FakeBox<T> {
            FakeBox(x)
        }
    }

    use std::ops::Deref;
    impl<T> Deref for FakeBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    let x = 5;
    let y = FakeBox::new(x);
    assert_eq!(5, *y);

    // deref coercions
    fn all_functions_perform_deref_coercions(arg: &str) {
        println!("Hello, {}", arg);
    }

    let m = FakeBox::new(String::from("Rust"));
    all_functions_perform_deref_coercions(&m); // even though the types don't match, it still works: because rust dereferenced the neccessary amount of times

    // DerefMut is the trait for mutable types

    // Drop trait (similar to Destructor)

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data {}!", self.data);
        }
    }

    {
        let x: CustomSmartPointer;
        // x.drop(); // illegal to call drop explicitly
        // but there is std::mem::drop -> called with drop(x)
    } // destructed

    // Rc<T> or Reference counting (similar to shared_pointers)

    // only suited for single-thread usage

    use std::rc::Rc;
    enum ListRC {
        ConsRC(i32, Rc<ListRC>),
        NilRC,
    }

    use ListRC::{ConsRC, NilRC};

    let a = Rc::new(ConsRC(5, Rc::new(ConsRC(10, Rc::new(NilRC)))));
    let b = ConsRC(3, Rc::clone(&a)); // clones increase reference count
    let c = ConsRC(4, Rc::clone(&a)); // bot b and c own a
    println!("reference count: {}", Rc::strong_count(&a));

    // Interior Mutability Pattern and RefCell<T>

    // RefCell is implemented using unsafe code, because it diregardes the borrowing rules
    // but it checks at runtime

    // Compiler can't ALWAYS know if code is safe (Halting Problem), therefor it might reject a correct program
    // -> solution if programmer knows it's he can use RefCell

    // example: test double or Mock object

    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.0 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of oyu quota");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sens_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    // having multiple owners of mutable data / combining Rc<T> and RefCell<T>
    let var_with_multiple_owners: Rc<RefCell<i32>>;
}


use std::thread;
use std::sync::{mpsc, Mutex, Arc};
fn channeling() {
    // multiple producer single consumer
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    // closure with `move` keyword takes ownership of captured variables
    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        thread::sleep(std::time::Duration::from_millis(1));
    });
    let received = rx.recv().unwrap();
    for received in rx {}
    handle.join().unwrap();
}

fn mutexing() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
