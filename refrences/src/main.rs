// In order to prevent a function from taking owenership of something
// like a string, we can specify that we want to take only the reference
// of the string and also

fn main() {
	let s1 = String::from( "foobar" );
	let len = calc_length( &s1 );      // passes the string, s1, by reference
	                                   // ( read by value ) to the function
									   // `calc_length`

	println!( "The length of '{}' is {}", s1, len );


	// the code bellow passes the reference to `add_bar` with the mut
	// flag/operater so that we can change s2, but retain owenership
	let mut s2 = String::from( "foo" );

	print!( "Change '{}'", s2 );

	add_bar( &mut s2 );

	println!( " to '{}'", s2 );
}

fn calc_length( s: &String ) -> usize { // takes a string by reference
	s.len( )
} // returns the length of the string

fn add_bar( some_string: &mut String ) {
	some_string.push_str( "bar" );
}
