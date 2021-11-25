struct Fruit {
    color:i32
}
impl Fruit {
    fn get(&self) -> i32 {
        self.color
    }
    fn set(&mut self,  value: &mut i32) {
        println!("set *value={}, value={}", *value, value);
        if *value>10 {
            *value = 10;
        }
        self.color = *value;
    }
}
fn main() {
    let mut array: [i32; 3] = [0; 3];
    let mut f = Fruit{color:3};
    let mut i :i32 = 899;
    f.set(&mut i);
    // array[4]=10;
    println!("Hello, world i = {}, f={}!", i, f.get());
}
