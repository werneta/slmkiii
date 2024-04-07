use slmkiii::{controls, template};

fn main() {
    let tmpl = template::default();

    println!("{:#?}", tmpl);
    println!("{:#?}", controls::serialize(&tmpl.rotary_controls[0]));
}
