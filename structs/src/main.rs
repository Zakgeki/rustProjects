fn main( ) {
	let mut user1 = User {
		email:       String::from( "joeschmoe@foobar.net" ),
		username:    String::from( "joeschmoe29" ),
		active:      true,
		login_count: 1,
	};

	let user2 = create_user( String::from( "jonny@foobar.net" ),
	                         String::from( "Jonny" ) );

	println!( "User1's email is {}", user1.email );

	user1.email = String::from( "joeschmoe2@foobar.net" );

	println!( "User1's email change to {}", user1.email );

	let black  = Color( 0, 0, 0 );
	let origin = Point( 0, 0, 0 );
}

// denomstrating tupled structs to create a new type
struct Color( i32, i32, i32 );
struct Point( i32, i32, i21 );

struct User {
	username:    String,
	email:       String,
	login_count: u64,
	active:      bool,
}

// fn create_user( email: String, username: String ) -> User {
// 	User {
// 		email:       email,
// 		username:    username,
// 		active:      true,
// 		login_count: 1,
// 	}
//
// }

// this version uses field init shorthand
fn create_user( email: String, username: String ) -> User {
	User {
		email,
		username,
		active: true,
		login_count: 1,
	}
}
