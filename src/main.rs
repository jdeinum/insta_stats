use followers::run;

fn main() {
    if let Err(e) = run() {
        println!("Error running script: {e:?}");
    }
}
