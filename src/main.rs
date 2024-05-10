use rs2048::game_frontend::RS2048;
use rs2048::game_backend::Backend;

fn main() -> iced::Result {
    iced::run(
        "2048",
        RS2048::update,
        RS2048::view
    )
}
