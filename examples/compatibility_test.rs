use flowerpassword::fp_code;

fn main() {
    println!("Rust outputs:");
    println!(
        "  password, key, 16: {}",
        fp_code("password", "key", 16).unwrap()
    );
    println!(
        "  test, github.com, 16: {}",
        fp_code("test", "github.com", 16).unwrap()
    );
    println!(
        "  mypassword, example.com, 12: {}",
        fp_code("mypassword", "example.com", 12).unwrap()
    );
}
