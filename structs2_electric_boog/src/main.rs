fn main() {
	let width1  = 30;
	let height1 = 50;

	println!( "The areo of the rectange is {} square pixels,",
	          area( width1, height1 ) );
}

fn area( width: u32, height: u32 ) -> u32 {
	width * height
}
