# String-algorithms-in-Rust
Various string/parsing algorithms covered in COMP261, rewritten in Rust from the original Java.  

To run:

1. Ensure you have an up-to date version of Rust. If you haven't installed Rust before, install the correct version for your operating system from the [offical site][1].
2. Open your terminal, and navigate to a directory where you don't mind downloading the contents of this GitHub repository. 
In that directory, run the command `git clone https://github.com/UncountableHollows/String-algorithms-in-Rust.git`. This command will download all the code in this repository into your computer in the directory you've chosen.
3. Now, in the directory you used `git clone`, you should have some directories with names like KMP Search, etc. In your terminal, navigate to one of these directory that
you wish to run.
4. Run the `cargo run` command to run the code in the directory. Some of the different algorithms will take multiple arguments, for example, to run the KMP algorithm to find the first occurence of the string "ABCD" in the first test file, you would run `cargo run tests/test1.txt ABCD`.


[1]: <https://www.rust-lang.org/tools/install> "Install Rust"