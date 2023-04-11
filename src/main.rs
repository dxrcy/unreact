use unreact::{object, run};

fn main() {
    let article = object! {
        one: "number one!",
        two: "number two!",
    };
    
    let a = object! {
        one: "number one!",
        two: "number two!",
    };
    
    let c = object! {
        one: "number one (c)!",
        two: "number two (c)!",
    };

    let values = object! {
        article,
        a,
        c,
        secret: "fart"
    };

    run(values).expect("Failed to run");
}
