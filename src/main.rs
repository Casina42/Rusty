use log::{info,error,warn,debug,trace,LevelFilter};
use env_logger;

fn just_fortytwo()->u8{
	42
}

struct ABC{
	a:u128,
	b:String,
}

fn return_abc()->ABC{
	ABC { a: 500, b: "Ehilà".to_string() }
}

fn main() {
	env_logger::Builder::new()
		.filter_level(LevelFilter::Trace)
		.init();

	info!("Hello, world!");
	let appoggio:ABC = return_abc();
	debug!("{}:{}",appoggio.a,appoggio.b);
	warn!("I don't feel very good world: my fever is {}°C!",just_fortytwo());
	error!("goodbye world");
	trace!("ghooooost");
}