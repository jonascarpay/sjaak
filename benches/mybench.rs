use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use shah::{
    bitboard::BitBoard,
    coord::{File, Rank, Square},
    pieces::{bishop, rook},
};

fn bishop(c: &mut Criterion) {
    c.bench_function("bishop ref", |b| {
        b.iter(|| {
            bishop::bishop_moves_ref(
                black_box(Square::from_coord(File::FA, Rank::R1)),
                black_box(BitBoard::from_bits(0x123123123)),
            )
        });
    });
    c.bench_function("bishop magic", |b| {
        b.iter(|| {
            bishop::bishop_moves_magic(
                black_box(Square::from_coord(File::FA, Rank::R1)),
                black_box(BitBoard::from_bits(0x123123123)),
            )
        });
    });
    c.bench_function("bishop magic unsafe", |b| {
        b.iter(|| {
            bishop::bishop_moves_magic_unsafe(
                black_box(Square::from_coord(File::FA, Rank::R1)),
                black_box(BitBoard::from_bits(0x123123123)),
            )
        });
    });
}

fn rook(c: &mut Criterion) {
    c.bench_function("rook ref", |b| {
        b.iter(|| {
            rook::rook_moves_ref(
                black_box(Square::from_coord(File::FA, Rank::R1)),
                black_box(BitBoard::from_bits(0x123123123)),
            )
        });
    });
    c.bench_function("rook magic", |b| {
        b.iter(|| {
            rook::rook_moves_magic(
                black_box(Square::from_coord(File::FA, Rank::R1)),
                black_box(BitBoard::from_bits(0x123123123)),
            )
        });
    });
    c.bench_function("rook magic unsafe", |b| {
        b.iter(|| {
            rook::rook_moves_magic_unsafe(
                black_box(Square::from_coord(File::FA, Rank::R1)),
                black_box(BitBoard::from_bits(0x123123123)),
            )
        });
    });
}

criterion_group!(benches, bishop, rook);
criterion_main!(benches);
