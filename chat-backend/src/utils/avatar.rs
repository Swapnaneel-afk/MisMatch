use rand::Rng;

pub fn generate_avatar_url(username: &str) -> String {
    // Placeholder function that would normally connect to an avatar service
    // For now, we'll just generate a URL to ui-avatars.com which creates avatars from initials
    let mut rng = rand::thread_rng();
    let color = format!("{:06x}", rng.gen::<u32>() & 0xFFFFFF);
    
    format!(
        "https://ui-avatars.com/api/?name={}&background={}&color=fff",
        urlencoding::encode(username),
        color
    )
}