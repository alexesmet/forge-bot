use opencv::highgui;
use std::time::Instant;

mod bot;
mod vision;
mod conf;

fn main() {
    let config = conf::Config::default();

    let mut bot = bot::Bot::new(config);
    let mut visor = vision::Visor::new(config).unwrap();
    let screen = visor.analyze_screen().unwrap();
    let field = visor.get_field_for_screen(&screen).unwrap();
    println!("{:?}", field);
    bot.field = field;
    bot.create_order(5, [2,2]);
    let mut last_inst = Instant::now();
    while bot.tasks.len() > 0 {
        if last_inst.elapsed().as_millis() > 900 {
            last_inst = Instant::now();
            bot.yelid_task();
        } else {
            std::thread::yield_now();
        }
    }

}


