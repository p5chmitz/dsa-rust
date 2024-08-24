mod maw;
mod sandbox;
mod tgg;

pub use tgg::{tgg_04, tgg_05};

fn main() {
    // Weiss
    ////////

    maw::maw_01::recursion(420);
    binary_search_driver();

    // Tamassia, Goodrich, and Goldwasser
    /////////////////////////////////////

    // Ch 4
    ///////

    // Compares two vectors for uniqueness
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    if tgg_04::unique_0(&a, &b) {
        println!("They're unique!");
    } else {
        println!("They are NOT unique");
    }

    // Searches a single array for unique elements
    let u = vec![4, 3, 9, 34, 1, 45, 23, 23];
    if tgg::tgg_04::unique_1(&u) {
        println!("This array comprises all unique values")
    }
    if tgg::tgg_04::unique_3(&u) {
        println!("This array comprises all unique values")
    }

    // Calculates prefix averages on an array of floats
    let c = vec![1.0, 3.0, 5.0, 7.0];
    let prefix_avg = tgg::tgg_04::prefix_average_0(&c);
    println!("{:?}", prefix_avg);
    let prefix_avg = tgg::tgg_04::prefix_average_1(&c);
    println!("{:?}", prefix_avg);

    // Ch 5
    ///////

    // Recursive approach to factorials
    let n = 4;
    println!("{}! = {}", n, tgg::tgg_05::factorial_0(n));
    let m = n as i32;
    println!("Teh book sex {}! = {}", m, tgg::tgg_05::factorial_1(m));
    println!("Refactor: {}! = {}", n, tgg::tgg_05::factorial_2(n));

    // Iterative approach to factorials
    println!("Iterative: {}! = {}", n, tgg::tgg_05::factorial_3(n));
    println!(
        "Iterative refactor: {}! = {}",
        n,
        tgg::tgg_05::factorial_4(n)
    );

    // Binary search using recursion
    // Binary search requires a sorted array of unique items.
    let a = vec![1, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 4;
    let i = tgg::tgg_05::bin_search_0(&a, target, 0, (a.len() - 1) as i32);
    if i >= 0 {
        println!("The target {} exists at index {}", target, i)
    } else {
        println!("The target {} does not exist within the array.", target)
    }

    // Sums the values of an array
    let v = vec![1, 2, 4, 8, 16];
    let sum = tgg::tgg_05::array_sum_0(v);
    println!("The sum of the values in the array: {sum}");

    // Reverses the elements of an array
    // Iterative implementation
    let again = vec![55, 66, 77, 88, 99];
    tgg_05::array_reversal_0(again);
    // Recursive impelmentation
    let mut again = vec![55, 66, 77, 88, 99];
    println!("Recursive approach\nOriginal: {:?}", again);
    let high = again.len() as i32 - 1; // Capture the # of indexes, not elements
    let recursive = tgg_05::array_reversal_1(&mut again, 0, high);
    println!("Reversed: {:?}", recursive);

    let n = 47;
    let seq = tgg::tgg_05::fib_0(n);
    println!("Fib attempt: Sequence of {} elements: {:?}", n, seq);

    // Ch 5 Extra Credit
    //------------------

    // Tower of Hanoi problem
    //tgg::tgg_05::hanoi_0(4);
    //tgg::tgg_05::tower_of_hanoi(6, 'a', 'b', 'c');

    // Dir printer
    let path = std::path::Path::new("src");
    tgg::tgg_05::disk_usage(path);

    // Ch 6
    ///////

    tgg::tgg_06::stack_0();

    // Sandbox runner
    /////////////////

    //sandbox::lists::list_adt_driver_0();
    sandbox::lists::list_adt_driver_1();
}

fn binary_search_driver() {
    let target = 73;
    let array: [i32; 39] = [
        1, 4, 5, 6, 10, 12, 16, 21, 23, 24, 25, 27, 31, 32, 33, 35, 37, 39, 40, 41, 42, 43, 45, 47,
        49, 50, 51, 52, 54, 56, 57, 60, 61, 67, 70, 71, 72, 73, 74,
    ];
    match maw::maw_01::binary_search(&array, target) {
        Some(index) => {
            println!(
                "Given an array of {} indexes, the target {} exists at index {}.",
                array.len(),
                target,
                index
            );
        }
        None => println!("The target {target} is not in the array"),
    };
}
