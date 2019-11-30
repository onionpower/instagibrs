pub fn run() {
    println!("hello from run");
    println!("{} likes {}", "betty", "edward");
    println!("{0} likes {0} and {1}", "betty", "tom");
    println!("{betty} likes {betty} and {tom}", betty="Betty", tom = "Tom");
    println!("bin: {:b} hex: {:x} o: {:o} d: {}", 10, 10, 10, 10);
    println!("{:?}", (12, true, "hello"));
    println!("10 + 10 = {}", 10+10)
}