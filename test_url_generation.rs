// Test file for URL generation logic
// This simulates the browser environment to test get_base_url function

use std::collections::HashMap;

// Mock window location for testing
struct MockLocation {
    hostname: String,
    protocol: String,
    port: String,
}

impl MockLocation {
    fn new(hostname: &str, protocol: &str, port: &str) -> Self {
        Self {
            hostname: hostname.to_string(),
            protocol: protocol.to_string(),
            port: port.to_string(),
        }
    }
}

// Simplified version of get_base_url for testing
fn get_base_url_test(location: &MockLocation) -> String {
    // Ensure protocol includes // for proper URL formatting
    let protocol = if location.protocol.ends_with(':') {
        format!("{}//", location.protocol)
    } else {
        location.protocol.clone()
    };
    
    match location.hostname.as_str() {
        "127.0.0.1" | "localhost" => {
            format!("http://{}:8000", location.hostname)
        }
        ip if ip.parse::<std::net::IpAddr>().is_ok() => {
            format!("http://{}:8000", ip)
        }
        domain => {
            match location.port.as_str() {
                "" | "80" | "443" => {
                    format!("{}{}", protocol, domain)
                }
                port => {
                    format!("{}{}:{}", protocol, domain, port)
                }
            }
        }
    }
}

fn main() {
    println!("Testing URL generation logic:\n");
    
    // Test cases
    let test_cases = vec![
        MockLocation::new("localhost", "http:", "8000"),
        MockLocation::new("127.0.0.1", "http:", "8000"),
        MockLocation::new("192.168.17.24", "http:", "8000"),
        MockLocation::new("kitchen-box.joes-web.de", "https:", ""),
        MockLocation::new("kitchen-box.joes-web.de", "https:", "443"),
        MockLocation::new("example.com", "http:", "8080"),
    ];
    
    for (i, location) in test_cases.iter().enumerate() {
        let result = get_base_url_test(location);
        println!("Test {}: {} -> {}", i + 1, location.hostname, result);
    }
    
    println!("\nExpected results:");
    println!("1. http://localhost:8000");
    println!("2. http://127.0.0.1:8000");
    println!("3. http://192.168.17.24:8000");
    println!("4. https://kitchen-box.joes-web.de");
    println!("5. https://kitchen-box.joes-web.de");
    println!("6. http://example.com:8080");
}
