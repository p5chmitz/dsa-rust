use std::path::Path;

// Initially appears to run in O(n^2) time, but actually
// runs in O(n) time because the algorithm touches (and performs O(1) operations)
// on n nodes in the tree exactly once. This algorithm represents multiple
// recursion because for each invocation there are x number of directory nodes to
// sum.
/** Walks a directory tree in O(n) time, prints names and sizes in in bytes (B) */
pub fn disk_usage(root: &Path) -> u64 {
    let mut dir_size = 0;
    if root.is_dir() {
        for e in root.read_dir().expect("read_dir call failed") {
            let entry = e.expect("failure to deconstruct value");
            dir_size += disk_usage(&entry.path()); // Recursive call
        }
        let this_dir = std::fs::metadata(root)
            .expect("metadata call failed [0]")
            .len();
        println!("D {:>7}B  {}", dir_size + this_dir, root.display());
    } else if root.is_file() {
        // Base case
        let size = std::fs::metadata(root)
            .expect("metadata call failed [1]")
            .len();
        println!("  {:>7}B  {}", size, root.display());
        return size;
    }
    dir_size
}
