use log::{info,warn,LevelFilter};
use env_logger;
use std::{sync::mpsc, thread::{self}, time::Duration};
use std::env;

struct ABC{
	a:usize,
	b:String,
}

fn return_abc(in_value:usize)->Result<ABC,String>{
	if in_value%5==0 || in_value%2==0{
		Err("Aiuto".to_string())
	} else{
		Ok(ABC{ a: in_value, b: "Ehilà".to_string() })
	}
}

fn main() {
	let argomenti:Vec<String> = env::args().collect();
	env_logger::Builder::new()
		.filter_level(LevelFilter::Trace) // this is necessary to show to console every log above the trace
		.init();

	let (upstream,downstream) = mpsc::channel();

	let handle= thread::spawn(move || {
		for i in 1..argomenti.len(){
			_ = upstream.send(ABC{a:i,b:argomenti[i].to_string()});
			match return_abc(i){
				Ok(appoggino)=>{
				_ = upstream.send(appoggino);
				},
				Err(erroraccio)=>warn!("{erroraccio}")
			};
			thread::sleep(Duration::from_millis(500));
		}
	});

	for rec in downstream{
		info!("GOT: {}:{}",rec.a,rec.b);
	}

	handle.join().unwrap();
}