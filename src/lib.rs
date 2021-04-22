/*
    Structure for a Rust library:
    lib.rs which specifies the modules
    One file for each module

    If you don't have a lib.rs, you're not creating a library,
    maybe you're just creating a binary
    (if you just have main.rs)

    In addition to library functionality, you can create separate binaries,
    in different places:
    - main.rs
    - src/bin/
    - examples/
    (can be customized in Cargo.toml)

    Conceptually:
    - library code goes in src (usable by other rust code)
    - binaries are compiled separately, possibly using the library code.
*/

pub mod popular;

/*
    You can also make modules without having a separate file

    I prefer (for the sake of standardization) to have one file per module
*/

pub mod lecture9_example {
    pub fn example_fn() {
        println!("Hello, world!");
    }
}
// ^ similar to having a file lecture9_example.rs
// if you have a lot of these, may be easier to maintain this way
// or if you want similar functionality to namespaces

/*
    Binaries: put binaries in src/main.rs, src/bin/<name>.rs, or examples/<name>.rs
    All of these more or less equivalent to the compiler but conceptually different
*/

/*
    Even more organization: subfolders which define sub-modules

    mod.rs file inside a folder indicates that it's a module with sub-modules

    Example: search
*/

pub mod search;

#[test]
fn test_submodule() {
    let bfs = search::bfs::BFS(3);
    assert_eq!(bfs.0, 3);
}
