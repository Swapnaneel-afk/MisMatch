pub fn generate_avatar_url(username: &str) -> String {
    format!(
        "https://ui-avatars.com/api/?name={}&background=random",
        urlencoding::encode(username)
    )
}