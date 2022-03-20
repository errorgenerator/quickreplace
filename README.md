# quickreplace - a CLI tool written in Rust

**quickreplace allows you to replace every occurence of a string of your choosing within a file and 
write the changed text to an output file**

This code originates from the example found in the book:

**[Programming Rust - Fast, Safe Systems Development](https://www.amazon.com/Programming-Rust-Fast-Systems-Development/dp/1492052590)**

*- by Jim Blandy, Jason Orendorff and Leonora F.S. Tindall.*

This particular example can be found in *Chapter 2: A tour of Rust - Filesystems and Command-Line Tools*

## Building and running:
<details>
<summary>CLICK TO EXPAND</summary>

### Requirements:
* rustup - with any toolchain installed.
    * rustc and cargo required.

### Building:
1. clone this repo

```git clone https://github.com/errorgenerator/quickreplace.git```

2. change into the directory

```cd ./quickreplace ```

3. build the code with cargo

```cargo build --release```

4. the compiled binary can be found in the newly created ```target/release``` directory.

```cd ./target/release```

5. you can now run the binary with ```./quickreplace```

</details>

### USAGE:
```Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>```

#### quickreplace expects 4 Arguments:
1. ```target``` - the String to search for.
2. ```replacement``` - the String to replace ```target``` with
3. ```INPUT``` - the path to a file containing the contents to filter.
3. ```OUTPUT``` - the path to a file to write the filtered contents to.

**Examples:**

* Replace every occurence of "World" in ```Hello.txt``` with "Rust" and write the output to ```Hello_Copy.txt```:

```quickreplace "World" "Rust" ./Hello.txt ./Hello_Copy.txt```

* Replace every occurence of "F" in ```Grades.csv``` with "A" and write the filtered contents back into ```Grades.csv```:

```quickreplace "F" "A" ./Grades.csv ./Grades.csv```