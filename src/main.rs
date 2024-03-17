use slmkiii::controls::defaults;

fn main() {
    for n in 0..16 {
        let rot = defaults::rotary(n);
        println!("{:#?}", rot);
    }
}
