use serde_json::json;

use unreact::run;

fn main() {
    let articles = json!({
        "one": "number one",
        "two": "number two",
    });

    let values = json!({
        "articles": articles,
    });

    let global = json!({
        "secret": "fart"
    });

    run(values, global).expect("Failed to run");
}
