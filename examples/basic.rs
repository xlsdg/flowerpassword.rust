use flowerpassword::fp_code;

fn main() {
    println!("Flower Password Generator - Basic Example\n");

    // Example 1: Basic usage
    println!("Example 1: Generate password for GitHub");
    match fp_code("my_master_password", "github.com", 16) {
        Ok(password) => println!("  Password: {}\n", password),
        Err(e) => eprintln!("  Error: {}\n", e),
    }

    // Example 2: Different lengths
    println!("Example 2: Different password lengths");
    let master = "my_secret_password";
    let service = "example.com";

    for length in [8, 12, 16, 24, 32] {
        match fp_code(master, service, length) {
            Ok(password) => println!("  Length {}: {}", length, password),
            Err(e) => eprintln!("  Error: {}", e),
        }
    }
    println!();

    // Example 3: Multiple services
    println!("Example 3: Same master password, different services");
    let master = "my_master_password";
    let services = vec![
        "github.com",
        "google.com",
        "twitter.com",
        "facebook.com",
    ];

    for service in services {
        match fp_code(master, service, 16) {
            Ok(password) => println!("  {}: {}", service, password),
            Err(e) => eprintln!("  {}: Error - {}", service, e),
        }
    }
    println!();

    // Example 4: Error handling
    println!("Example 4: Error handling");
    println!("  Trying length 1 (too short):");
    match fp_code("password", "key", 1) {
        Ok(pwd) => println!("    Password: {}", pwd),
        Err(e) => println!("    Error: {}", e),
    }

    println!("  Trying length 50 (too long):");
    match fp_code("password", "key", 50) {
        Ok(pwd) => println!("    Password: {}", pwd),
        Err(e) => println!("    Error: {}", e),
    }
}
