use rs2048::game_frontend::RS2048;
use rs2048::game_backend::Backend;

fn main() -> iced::Result {
    iced::program(
        "2048",
        RS2048::update,
        RS2048::view
    )
        .subscription(rs2048::game_frontend::RS2048::keyboard_subscription)
        .run()
}
