#![allow(unused)]

extern crate pkg_config;

fn main() {
    pkg_config::probe_library("vips");
}
