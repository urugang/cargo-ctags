# cargo-ctags 
[![Build Status](https://travis-ci.org/urugang/cargo-ctags.svg?branch=master)](https://travis-ci.org/urugang/cargo-ctags) 
[![Coverage Status](https://coveralls.io/repos/github/urugang/cargo-ctags/badge.svg?branch=master)](https://coveralls.io/github/urugang/cargo-ctags?branch=master)

## universal-ctags 
install ctags from <https://github.com/universal-ctags/ctags.git>   
## cargo-ctags
usage: cargo ctags [cargo metadata arguments] -- [ctags arguments]   

`
~/codes/cargo-ctags $ cargo install cargo-ctags  
`

`
~/codes/cargo-ctags $ cargo ctags -- -e # TAGS for emacs   
~/codes/cargo-ctags $ ls -l TAGS  
-rw-rw-r--   1 urugang        urugang   1415698 2017-05-31 08:13 TAGSc  
`

`
~/codes/cargo-ctags $ cargo ctags -- # TAGS for vi   
~/codes/cargo-ctags $ ls -l tags  
-rw-rw-r--   1 urugang        urugang   1415698 2017-05-31 08:14 tagsc  
`

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
