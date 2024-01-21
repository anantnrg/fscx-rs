use fscx_rs::dir::copy;
use fscx_rs::dir::traverse;
use fscx_rs::Progress;

fn main() {
    let paths = traverse(
        "/home/mik3y/projects/repos/video-scripts",
        vec!["./examples", "target", ".git", "examples/test.txt"],
    )
    .unwrap();

    let res = copy(
        "/home/mik3y/Downloads",
        "./tests",
        vec!["target"],
        true,
        Some(|progress: Progress| {
            println!(
                "\rCopying: {}% ({}/{})",
                progress.percentage, progress.processed_bytes, progress.total_bytes
            );
        }),
        None,
    );
    println!("{:?}", res);
}
