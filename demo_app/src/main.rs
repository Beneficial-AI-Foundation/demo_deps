use my_lib::{calculator, string_utils, greet};

fn main() {
    println!("=== Demo App using my_lib ===");
    
    // Test greeting functionality
    println!("{}", greet("Rust Developer"));
    
    // Test calculator functions
    println!("\n--- Calculator Tests ---");
    println!("2 + 3 = {}", calculator::add(2, 3));
    println!("4 * 5 = {}", calculator::multiply(4, 5));
    println!("2^5 = {}", calculator::power(2, 5));
    
    // Test string utilities
    println!("\n--- String Utilities Tests ---");
    let sample_text = "hello world rust programming";
    println!("Original: '{}'", sample_text);
    println!("Reversed: '{}'", string_utils::reverse_string(sample_text));
    println!("Word count: {}", string_utils::count_words(sample_text));
    println!("Title case: '{}'", string_utils::to_title_case(sample_text));
    
    println!("\n--- Demo completed successfully! ---");
}
