mod maw;
mod tgg;

pub use tgg::{tgg_04, tgg_05};

fn main() {

    // Weiss
    ////////

    maw::maw_01::recursion(420);
    binary_search_driver();

    // Tamassia, Goodrich, and Goldwasser
    /////////////////////////////////////

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
