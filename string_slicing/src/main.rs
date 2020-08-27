fn main( ) {
	let s1 = String::from( "foobar consists of two parts, foo and bar." );

	let word = first_word( &s1 ); // word will be set to 6

	println!( "The length of the first word of '{}' is {}", s1, word );

	let foo = &s1[ 0..3 ];
	let bar = &s1[ 3..6 ];

	println!( "{}{}", foo, bar );

}

fn first_word( s: &String ) -> usize {
	let bytes = s.as_bytes( );

	for ( i, &item ) in bytes.iter( ).enumerate( ) {
		if item == b' ' {
			return i;
		}
	}

	s.len( )
}
