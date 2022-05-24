use hive::axial::Axial;
use rand::seq::IteratorRandom;
use rand::Rng;

use hive::board::Board;
use hive::tile::{Colour, TileType};

use alloc_counter::AllocCounterSystem;

#[global_allocator]
static A: AllocCounterSystem = AllocCounterSystem;

fn render_random_board() {
    let mut b = Board::default();
    let mut rng = rand::thread_rng();
    let mut colour = Colour::White;
    for i in 0..20 {
        if rng.gen::<bool>() {
            b.place(
                (colour, TileType::Ant),
                b.get_available_placements(colour).choose(&mut rng).unwrap(),
            );

            colour = colour.other();
        }
    }

    b.render_stdout();
}

fn main() {
    let mut b = Board::default();

    let (counts, _) = alloc_counter::count_alloc(|| {
        b.place((Colour::White, TileType::Ant), Axial::zero());
        b.place((Colour::Black, TileType::Ant), Axial::zero().north());
        b.place((Colour::White, TileType::Ant), Axial::zero().south());
        b.place(
            (Colour::Black, TileType::Ant),
            Axial::zero().north().north(),
        );
        b.place(
            (Colour::White, TileType::Ant),
            Axial::zero().south().south(),
        );
        b.place(
            (Colour::Black, TileType::Ant),
            Axial::zero().north().north().north(),
        );
        b.place(
            (Colour::White, TileType::Ant),
            Axial::zero().south().south().south(),
        );

        let white_placements = b.get_available_placements(Colour::White);
        println!(
            "white_placements size_hint : {:?}",
            white_placements.size_hint()
        );
        println!(
            "white placements vec size {}",
            white_placements.collect::<Vec<_>>().len()
        );

        println!(
            "{:?}",
            b.get_available_placements(Colour::Black).size_hint()
        );
    });

    b.render_stdout();

    println!("total counts {:?}", counts);
}
