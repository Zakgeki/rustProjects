use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{
    println!("Guess the number!");

    loop
    {
        println!( "Please input your guess." );

        let mut guess = String::new( );

        io::stdin( )
            .read_line( &mut guess )
            .expect( "Failed to read line" );

        let guess: u32 = match guess.trim( ).parse( )
        {
            Ok( num ) => num,
            Err( _ )  => continue,
        };


        let secret_num = rand::thread_rng( ).gen_range( 1, 101 );

        println!( "You guessed: {}", guess );

        print!( "The Secret Number was {}. ", secret_num );
    
        match guess.cmp( &secret_num )
        {
            Ordering::Less    => println!( "Too Small!" ),
            Ordering::Greater => println!( "Too Big!"   ),
            Ordering::Equal   => 
            { 
                println!( "You win!"   );
                break;
            }
        }
    }
}
