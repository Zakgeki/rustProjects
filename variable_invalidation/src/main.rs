fn main() {
    let s1 = String::from( "foo" );
	let s2 = s1;

	println!( "{}bar", s2 );
	// the statement bellow is illegal because the statement `let s2 = s1;`
	// moves s1 to s2 and invalidates it to prevent a double free error
	// println!( "{}bar", s1 );
}
