use log::{info,error,warn,debug,trace,LevelFilter};
use env_logger;
use std::{thread::{self, JoinHandle}, time::Duration, vec};

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

	let mut handles:Vec<JoinHandle<()>> = vec![];
	for i in 1..10{
		let handle= thread::spawn(move || {
				match return_abc(i){
					Ok(appoggino)=>debug!("{}:{}",appoggino.a,appoggino.b),
					Err(erroraccio)=>error!("{erroraccio}")
				}
		
		});
		handles.push(handle);
		thread::sleep(Duration::from_millis(500));
	}

	info!("Hello, world!");
	warn!("I don't feel very good world: my fever is {}°C!",just_fortytwo());
	error!("goodbye world");
	trace!("ghooooost");

	for h in handles{
		h.join().unwrap();
	}
}