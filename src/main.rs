
mod bot;
mod vision;


fn main() {
    let mut Bot = bot::Bot::new(bot::ScreenConfig::default());

    //Bot.place_ore(2, 1);
    vision::analyse_demo();
}


