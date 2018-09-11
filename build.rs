extern crate gcc;

fn main() {
    gcc::Build::new()
        .file("src/redismodule.c")
        .include("src/")
        .compile("libredismodule.a");
}