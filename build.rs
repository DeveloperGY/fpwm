fn main() {
    #[cfg(not(target_os = "linux"))]
    {
        println!("You are not using linux, fpwm does not support your system!");
        std::process::exit(-1);
    }
    
    println!("Building fpwm...");
}