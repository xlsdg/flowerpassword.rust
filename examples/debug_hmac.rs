use flowerpassword::fp_code;

fn main() {
    println!("Testing empty strings:");
    println!("Rust - password=\"\", key=\"\", length=16: {:?}", fp_code("", "", 16));
    println!("Rust - password=\"password\", key=\"\", length=16: {:?}", fp_code("password", "", 16));
}
