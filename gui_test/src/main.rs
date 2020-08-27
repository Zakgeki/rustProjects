#[ cfg( windows ) ] extern crate winapi;
use std::io::Error;

#[ cfg( windows ) ]
fn print_message( msg: &String ) -> Result<i32, Error > {
	use std::ffi::OsStr;
	use std::iter::once;
	use std::os::windows::ffi::OsStrExt;
	use std::ptr::null_mut;
	use winapi::um::winuser::{ MB_OK, MessageBoxW };
	let wide: Vec< u16 > = OsStr::new( msg ).encode_wide( ).chain( once( 0 ) ).collect( );
	let ret = unsafe {
		MessageBoxW( null_mut( ), wide.as_ptr( ), wide.as_ptr( ), MB_OK )
	};

	if ret == 0 { Err( Error::last_os_error( ) ) }
	else { Ok( ret ) }
}

#[ cfg( not( windows ) ) ]
fn print_message( msg: &String ) -> Result< (), Error > {
	println!( "{}", msg );
	Ok( () )
}

fn main() {
	let s1 = String::from( "foobar" );
	print_message( &s1 ).unwrap( );
}
