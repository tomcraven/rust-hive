use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use hive::{
    axial::Axial,
    board::Board,
    tile::{Colour, TileType},
};
use rand::seq::IteratorRandom;
use rand::Rng;

fn place(c: &mut Criterion) {
    c.bench_function("place only", |b| {
        b.iter_batched(
            || Board::default(),
            |mut board| board.place((Colour::Black, TileType::Ant), Axial::zero()),
            BatchSize::SmallInput,
        )
    });
}

fn get_available_placements(c: &mut Criterion) {
    c.bench_function("get_available_placements no tiles", |b| {
        b.iter_batched(
            || Board::default(),
            |board| {
                board
                    .get_available_placements(Colour::White)
                    .for_each(|_| ())
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("get_available_placements single tile", |b| {
        b.iter_batched(
            || {
                let mut b = Board::default();
                b.place((Colour::Black, TileType::Ant), Axial::zero());
                b
            },
            |board| {
                board
                    .get_available_placements(Colour::White)
                    .for_each(|_| ())
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("get_available_placements multiple tiles", |b| {
        b.iter_batched(
            || {
                let mut b = Board::default();
                let mut rng = rand::thread_rng();
                let mut colour = Colour::White;
                for _ in 0..20 {
                    if rng.gen::<bool>() {
                        let maybe_p = b.get_available_placements(colour).choose(&mut rng);
                        if let Some(p) = maybe_p {
                            b.place((colour, TileType::Ant), p);
                        } else {
                            println!();
                            b.render_stdout();
                            assert!(false);
                        }

                        colour = colour.other();
                    }
                }

                b
            },
            |board| {
                board
                    .get_available_placements(Colour::White)
                    .for_each(|_| ())
            },
            BatchSize::SmallInput,
        )
    });

    /*c.bench_function("get_available_placements multiple tiles v2", |b| {
        b.iter_batched(
            || {
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

                b
            },
            |board| {
                board
                    .get_available_placements_v2(Colour::White)
                    .for_each(|_| ())
            },
            BatchSize::SmallInput,
        )
    });*/
}

criterion_group!(benches, place, get_available_placements);
criterion_main!(benches);
