use std::time::Instant;

mod bot;
mod vision;


fn main() {
    let mut bot = bot::Bot::new(bot::ScreenConfig::default());
    bot.create_order(5, [3,3]);
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


