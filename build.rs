use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=input.css");
    
    // Exécute Tailwind
    let output = Command::new("npx")
        .args(["tailwindcss", "-i", "./input.css", "-o", "./dist/output.css"])
        .output()
        .expect("Failed to execute Tailwind");

    if !output.status.success() {
        panic!("Tailwind compilation failed: {}", 
            String::from_utf8_lossy(&output.stderr));
    }
}
