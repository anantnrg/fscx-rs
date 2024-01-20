use fscx_rs::dir::traverse;

fn main() {
    let paths = traverse(vec!["target", ".git", "examples/copy_dir.rs"]);

    println!("{:?}", paths);
}
