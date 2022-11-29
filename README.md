# <p align="center">`iterx`</p>

<p align="center">
    <a href="https://github.com/codereport/rust-iterx/issues" alt="contributions welcome">
        <img src="https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat" /></a>
    <a href="https://lbesson.mit-license.org/" alt="MIT license">
        <img src="https://img.shields.io/badge/License-MIT-blue.svg" /></a>    
    <a href="https://rust-lang.org/">
        <img src="https://img.shields.io/badge/Rust-ff69b4.svg"/></a>
    <a href="https://github.com/codereport?tab=followers" alt="GitHub followers">
        <img src="https://img.shields.io/github/followers/codereport.svg?style=social&label=Follow" /></a>
    <a href="https://GitHub.com/codereport/rust-iterx/stargazers/" alt="GitHub stars">
        <img src="https://img.shields.io/github/stars/codereport/rust-iterx.svg?style=social&label=Star" /></a>
    <a href="https://twitter.com/code_report" alt="Twitter">
        <img src="https://img.shields.io/twitter/follow/code_report.svg?style=social&label=@code_report" /></a>
</p>

`iterx` is a [Rust](https://rust-lang.org/) library that provides several functions on the [`Iterator`](https://doc.rust-lang.org/1.64.0/core/iter/trait.Iterator.html) trait not found in [`std::iter`](https://doc.rust-lang.org/1.64.0/core/iter/) or the [`Itertools`](https://docs.rs/itertools/latest/itertools/) crate.

Current functions provided:

* [`drop_last`](https://docs.rs/iterx/latest/iterx/trait.Iterx.html#method.drop_last)
* [`prescan`](https://docs.rs/iterx/latest/iterx/trait.Iterx.html#method.prescan)
* [`scan_`](https://docs.rs/iterx/latest/iterx/trait.Iterx.html#method.scan_)
* [`scan_while`](https://docs.rs/iterx/latest/iterx/trait.Iterx.html#method.scan_while)
* [`zip_map`](https://docs.rs/iterx/latest/iterx/trait.Iterx.html#method.zip_map)
