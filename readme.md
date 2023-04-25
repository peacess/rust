# Study Rust and Sample

## Generate "The Rust Programming Language" epub

cargo install mdbook  
cargo install mdbook-epub  
git clone https://github.com/rust-lang/book.git  
cd book  
add the following in the file "book.toml"   
"  
[output.epub]  
additional-css = ["ferris.css", "theme/2018-edition.css"]  
"  
mdbook build  
the epub file will be generated in folder book/epub  
