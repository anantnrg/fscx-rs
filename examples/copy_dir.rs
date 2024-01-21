use fscx_rs::dir::copy;
use fscx_rs::dir::traverse;
use fscx_rs::Progress;

fn main() {
    let paths = traverse(
        "./",
        vec!["./examples", "target", ".git", "examples/test.txt"],
    )
    .unwrap();

    copy(
        "./",
        "./tests",
        vec!["target"],
        false,
        Some(|progress: Progress| {
            println!(
                "\rCopying: {}% ({}/{})",
                progress.percentage, progress.processed_bytes, progress.total_bytes
            );
        }),
        None,
    )
    .unwrap();
    println!("{:?}", paths);
}
