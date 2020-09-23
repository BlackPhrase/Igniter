pub mod graphics;
pub mod sound;
pub mod input;

pub fn init()
{
	graphics::init();
	sound::init();
	input::init();
	
	println!("Initialized!");
}

pub fn shutdown()
{
	println!("Shutting down...");
	
	input::shutdown();
	sound::shutdown();
	graphics::shutdown();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
