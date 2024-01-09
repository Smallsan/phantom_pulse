pub fn bytes_to_gigabytes(bytes: u64) -> f64 {
    return (bytes as f64 / 1024_f64.powi(3)).ceil();
}
