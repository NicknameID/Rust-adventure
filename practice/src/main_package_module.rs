mod service;

use service::hello;

fn main() {
    hello::function("world!");
}