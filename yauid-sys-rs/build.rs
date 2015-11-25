extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("yauid/src/yauid.c")
        .include("yauid/api/")
        .compile("libyauid.a");
}

