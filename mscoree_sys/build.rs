extern crate cc;

fn main() {
    println!("cargo:rustc-link-lib=static=mscoree");
    println!("cargo:rustc-link-search=native=C:\\Program Files (x86)\\Windows Kits\\NETFXSDK\\4.6.1\\Lib\\um\\x64");
}