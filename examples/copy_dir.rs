use fscx_rs::dir::traverse;

fn main() {
    let paths = traverse(vec![
        "examples/copy_dir.rs",
        "target",
        ".git",
        "examples/test.txt",
    ]);

    println!("{:?}", paths);
}
