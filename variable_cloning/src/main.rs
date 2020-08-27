fn main() {
	let s1 = String::from( "foo" );
	let s2 = s1.clone( );

	// using the function *.colne( ) copies the string instead of meving it
    println!( "{}bar1, {}bar2", s1, s2 );
}
