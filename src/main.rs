slint::include_modules!();

use eyre::*;
use rand::seq::SliceRandom;
use slint::{Model, Timer, VecModel};
use std::{rc::Rc, time::Duration};

fn main() -> Result<()> {
    let ui = AppWindow::new()?;

    // Fetch the tiles from the model
    let mut tiles: Vec<TileData> = ui.get_memory_tiles().iter().collect();
    // Duplicate them to ensure that we have pairs
    tiles.extend(tiles.clone());

    // Randomly mix the tiles
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // Assign the shuffled Vec to the model property
    let tiles_model = Rc::new(VecModel::from(tiles));
    ui.set_memory_tiles(tiles_model.clone().into());

    let ui_weak = ui.as_weak();
    ui.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) =
            (flipped_tiles.next(), flipped_tiles.next())
        {
            if t1 == t2 {
                t1.solved = true;
                tiles_model.set_row_data(t1_idx, t1);
                t2.solved = true;
                tiles_model.set_row_data(t2_idx, t2);
            } else {
                let ui = ui_weak.unwrap();
                ui.set_disable_tiles(true);
                let tiles_model = tiles_model.clone();
                Timer::single_shot(Duration::from_secs(1), move || {
                    ui.set_disable_tiles(false);
                    t1.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    t2.image_visible = false;
                    tiles_model.set_row_data(t2_idx, t2);
                });
            }
        }
    });

    Ok(ui.run()?)
}
