fn main() {
    #[cfg(not(target_os = "linux"))]
    {
        eprintln!("fpwm is for unix/linux only");
        std::process::exit(-1);
    }

    eprintln!("Building fpwm...");
}