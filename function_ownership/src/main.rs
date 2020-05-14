fn main() {
	let s = String::from( "foo" ); // s comes inte scope

	takes_ownership( s );          // s's value moves into the function
	                               // and so is no longer valid in main( )

	let x = 5;                     // x comes into scope

	makes_copy( x );               // x would move into the function,
	                               // but i32 has the type trait Copy,
								   // therfore it's okay to use x afterawrd
								   // and is still in scope
} // X and s go out of scope, but becaue s's value was moved, noting special
  // happens.

fn takes_ownership( some_string: String ) { // some_string comes into scope
	println!( "{}", some_string );
} // some_string goes out of scope and `drop' is called.
  // The backing memory is freed`

fn makes_copy( some_integer: i32 ) { // some_integer comes into scope
	println!( "{}", some_integer );
} // someS_integer goes out of scope, but nothing happens
