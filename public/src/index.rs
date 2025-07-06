fn main(){
	let tupla: (i32, f64, bool, char) = (10, 3.14, true, 'p');

	println!("La primera variable es {}", tupla.0);
	println!("La segunda variable es {}", tupla.1);
	println!("La tercera variable es {}", tupla.2);
	println!("La ultima variable es {}", tupla.3);
}