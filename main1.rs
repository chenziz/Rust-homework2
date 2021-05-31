trait Light {
    fn print_time(&self);
}
struct Red {}
impl Light for Red {
    fn print_time(&self) {
        println!("5min");
    }
}
struct Yellow{}
impl Light for Yellow {
    fn print_time(&self) {
        println!("4min");
    }
}
struct Green{}
impl Light for Green {
    fn print_time(&self) {
        println!("7min");
    }
}
fn main() {
    let r = Red {};
    r.print_time();
    let g = Green {};
    g.print_time();
    let y = Yellow {};
    y.print_time();
}
