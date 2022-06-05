#[derive(Debug)]
struct Student {
	name: String,
	gpa: f32,
}
fn main() {

	let students = vec![
		"Bogdan 3.1",
		"Wallace 2.3",
		"Lidiya 3.5",
		"Kyle 3.9",
		"Anatoliy 4.0",
	];

	//Using an iterator, a type of combinator
	let good_students: Vec<Student> = students.iter()
	
		//Chain it with a map method, which takes a closure to transform the input parameter into another type, in this case an Optional.
		.map(|s| {
			let mut s = s.split(' ');
			let name = s.next()?.to_owned();
			let gpa = s.next()?.parse::<f32>().ok()?;

			Some(Student {name, gpa})
		})

		//Flatten combinator will consume the passed iterator and return a "flattened" version. In this case, it will resolve Optionals into either Some, or None. The latter will be discarded.
		.flatten()

		//The filter combinator will discard any Student that doesn't satisfy the closure
		.filter(|s| s.gpa >= 3.5)

		//And finally, the collect combinator will turn the iterator into a plain &Student list
		.collect();

	for s in good_students {
		println!("{:?}", s);
	}
	
}
