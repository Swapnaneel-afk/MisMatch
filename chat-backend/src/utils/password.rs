use rand::Rng;
use std::fmt::Write;

// Simple password hashing function (in real app, use a proper hashing library like bcrypt)
pub fn hash_password(password: &str) -> String {
    // Generate a random salt
    let mut salt = String::new();
    let mut rng = rand::thread_rng();
    
    for _ in 0..16 {
        let _ = write!(salt, "{:x}", rng.gen::<u8>());
    }
    
    // For demo purposes, just append the salt to the password and hash with a simple algorithm
    // In a real app, use a proper password hashing algorithm like bcrypt, Argon2, etc.
    let combined = format!("{}{}", salt, password);
    
    // Simple hash function (NOT SECURE - for demo only)
    let mut hash = 0u64;
    for byte in combined.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    
    format!("{}${:x}", salt, hash)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    if let Some(salt) = hash.split('$').next() {
        let combined = format!("{}{}", salt, password);
        
        // Apply the same hash function
        let mut computed_hash = 0u64;
        for byte in combined.bytes() {
            computed_hash = computed_hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Check if the computed hash matches the stored hash
        let expected_hash = format!("{}${:x}", salt, computed_hash);
        return expected_hash == hash;
    }
    
    false
} 