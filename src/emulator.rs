use ggez::{event::run, ContextBuilder};

use crate::{constants, debug::cpu_view::CpuView};

pub fn start() {
    let cb = ContextBuilder::new(
        constants::emulator::GAME_ID,
        constants::emulator::AUTHOR_NAME,
    );
    let (ctx, event_loop) = cb.build().unwrap();
    let view = CpuView::new().unwrap();

    run(ctx, event_loop, view);
}
