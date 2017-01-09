fn main() {
	let shrug_name = format!("Sheffield Ruby"); 
    greet(shrug_name);
}

fn greet(name: String) {
    println!("Hello {}!", name);
}
