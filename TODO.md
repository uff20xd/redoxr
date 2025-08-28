
```Rust
struct CodeObject {
  outpath: String,
  name: String,
  inputs: Box<[&str]>,
  command: Cmd,
}
struct RustCrate {
  ...
}
fn main () -> () {
  let redoxr = Redoxr::new(&[]); 
}
```
