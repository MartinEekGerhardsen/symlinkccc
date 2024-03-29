# symlinkccc

A tool to SYMbolically LINK Catkin Compile Commands. 

## Getting started

Install [rust](https://www.rust-lang.org/tools/install), and make sure it is in your PATH by adding 

```bash
test -f "${HOME}/.cargo/env" && source "${HOME}/.cargo/env" 
```

to your `.bashrc` file (or equivalent). 

Install this package using: 

```bash
cargo install symlinkccc
```

Then, in a `catkin` workspace: 

```bash
catkin config --cmake-args -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
catkin build 
symlinkccc
```

This command is pretty quick, especially compared to the average `catkin` build time, so I made a wrapper for `catkin`, which executes the `symlinkccc` program after each `catkin build`, while passing through any arguments. Add this to your `.bashrc` file (or equivalent). 

```bash
_catkin_wrapper () {
    if (( $# == 0 ))
    then
        \catkin 
    else
        if [[ $1 == "build" ]]
        then 
            \catkin build ${@:2} && symlinkccc
        else 
            \catkin $@
        fi
    fi
}

alias catkin="_catkin_wrapper"
```

## Why? 

ROS is an extremely useful framework when working anywhere near the robotics field, and `catkin` is an extremely useful set of tools for building and generally handling all the ROS packages. `clangd` is a very nice language server for the C++ programming language. However, `catkin` and `clangd` are not really friends...

`clangd` requires a `compile_commands.json`-file in order to understand which dependencies a package has. This can be generated by the `cmake` build tool, which `catkin` calls under the hood. To make `catkin` generate these files, add `-DCMAKE_EXPORT_COMPILE_COMMANDS=ON` to the `cmake` arguments, e.g. like this: 

```bash
catkin config --cmake-args -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
```

The problem is that catkin places the `compile_commands.json`-files in the build folder for each individual package, while `clangd` wants it in the root of the source folder for each package (as it is slightly more likely that you'll edit code from the source folder compared to the build folder). An advantage of ROS/`catkin` is that 

## TODOs
Some things I'd like to explore if I have time. I doubt too much of this will have an
impact on the actual performance.

 - [x] Fix warning due to workspace being the name of a mod and multiple other things
 - [ ] Use async
 - [ ] Async handling of source and build directories
 - [ ] Async loading files (package.xml/CMakeLists.txt)
 - [ ] Async linking of different files
 - [ ] Test if `cargo watch` can be used to check if something has to be re-linked
 - [x] Try to get the `rospack` command implemented purely in rust
 - [ ] See if links can be juggled for profiles rather than just removing the old link for every time the program is executed
 - [ ] Look to minimize the size of the binary by eliminating unneccessary dependencies. 
 - [ ] Can ROS compile on pure Windows? If so the symlink won't work.
