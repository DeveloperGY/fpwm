fn main() {
    #[cfg(not(target_os = "linux"))]
    {
        std::process::exit(-1);
    }
}