use std::io;

fn main( ) {
    // prompt user for temperature
    println!( "> Temperature conversion" );
    println!( "> What is the temperature you'd like to convert?\n" );

    // take in temperature
    let mut user_in  = String::new( );
    io::stdin( ).read_line( &mut user_in )
        .expect( "\n> Failed to read line" );

    let temprtr: f32 = user_in.trim().parse::<f32>( ).unwrap( );

	// prompt user for units
	let mut unit_loop = true;

	while unit_loop {
	    println!( "\n> Is your temp in (F)arenheit or (C)elsius?\n" );

		user_in = "".to_string( );

		io::stdin( ).read_line( &mut user_in )
	        .expect( "\n> Failed to read line" );

		if user_in.trim( ) == "C" || user_in.trim( ) == "c"  {
			let converted_temp = cel_to_faren( temprtr );
			println!( "\n> Your entered temp is {}C which is {}F", temprtr, converted_temp );
			unit_loop = false;
		}
		else if user_in.trim( ) == "F" || user_in.trim( ) == "f" {
			let converted_temp = faren_to_cel( temprtr );
			println!( "\n> Your entered temp is {}F which is {}C", temprtr, converted_temp );
			unit_loop = false;
		}
		else {
			println!( "\n> Error: Invalid units" );
		}
	}

	println!( "> Goodbye!" );
}

// calculate
fn faren_to_cel( temp: f32 ) -> f32 {
	( temp - 32.0 ) * ( 5.0 / 9.0 )
 }

 fn cel_to_faren( temp: f32 ) -> f32 {
	temp * ( 5.0 / 9.0 ) + 32.0
 }
