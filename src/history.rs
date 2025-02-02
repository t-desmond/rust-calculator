use std::fs::OpenOptions;

fn main(){
  let _file = OpenOptions::new()
  .read(true)
  .append(true)
  .write(true)
  .create(true)
  .open("history");
}