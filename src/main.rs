use log::{info,error,warn,debug,trace,LevelFilter};
use env_logger;
use std::{thread, time::Duration};

fn just_fortytwo()->u8{
	42
}

struct ABC{
	a:u128,
	b:String,
}

fn return_abc(in_value:u8)->Result<ABC,String>{
	if in_value%5==0 || in_value%2==0{
		Ok(ABC{ a: 500, b: "Ehilà".to_string() })
	}
	else{
		Err("Aiuto".to_string())
	}
}

fn main() {
	env_logger::Builder::new()
		.filter_level(LevelFilter::Trace)
		.init();

	let handle= thread::spawn( || {
		for i in 1..10{
			match return_abc(i){
				Ok(appoggino)=>debug!("{}:{}",appoggino.a,appoggino.b),
				Err(erroraccio)=>error!("{erroraccio}")
			}
			thread::sleep(Duration::from_millis(500));
		}
	});

	info!("Hello, world!");
	warn!("I don't feel very good world: my fever is {}°C!",just_fortytwo());
	error!("goodbye world");
	trace!("ghooooost");

	handle.join().unwrap();
}