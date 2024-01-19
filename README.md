# Rust Raptor

![Rust Build](https://github.com/santtuniskanen/rust-raptor/actions/workflows/rust_build.yml/badge.svg)

#### Rust Raptor is a Command Line tool emulating the classic grep tool from UNIX based systems. This is a project in [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) book.

## Goals
The aim of this project is to learn how to make Command Line applications with Rust, as well as writing tests and reading files. This project will never be as complete as [ripgrep](https://github.com/BurntSushi/ripgrep), but I'm aiming to add more features as I go along.

## Things I've done
So far I've added the following myself, I hope the list grows in the future:

* Modularization by splitting the project into smaller modules to prevent one file having a lot of different things.
* Input validation. The application now responds with proper error messages when the path or file don't exist, or if the given input isn't a file. (*This should be extended more in the future*)