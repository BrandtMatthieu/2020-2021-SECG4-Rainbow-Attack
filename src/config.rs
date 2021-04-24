pub struct Config {
    pub min_password_length: usize,
    pub max_password_size: usize,
    pub max_file_size: usize,
    pub hash_count_per_buffer: usize,
    pub thread_count: usize,
}
