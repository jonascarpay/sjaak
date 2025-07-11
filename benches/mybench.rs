use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use sjaak::{
    bitboard::BitBoard,
    coord::{File, Rank, Square},
    pieces::{
        bishop::{
            fast::{bishop_moves_magic, bishop_moves_magic_unsafe},
            reference::bishop_moves_reference,
        },
        rook::{
            fast::{rook_moves_magic, rook_moves_magic_unsafe},
            reference::rook_moves_reference,
        },
    },
};

fn bishop(c: &mut Criterion) {
    c.bench_function("bishop ref", |b| {
        b.iter(|| {
            bishop_moves_reference(
                black_box(Square::A1),
                black_box(BitBoard::from_bits(0x123123123)),
            )
        });
    });
    c.bench_function("bishop magic", |b| {
        b.iter(|| {
            bishop_moves_magic(
                black_box(Square::A1),
                black_box(BitBoard::from_bits(0x123123123)),
            )
        });
    });
    c.bench_function("bishop magic unsafe", |b| {
        b.iter(|| {
            bishop_moves_magic_unsafe(
                black_box(Square::A1),
                black_box(BitBoard::from_bits(0x123123123)),
            )
        });
    });
}

fn rook(c: &mut Criterion) {
    c.bench_function("rook ref", |b| {
        b.iter(|| {
            rook_moves_reference(
                black_box(Square::A1),
                black_box(BitBoard::from_bits(0x123123123)),
            )
        });
    });
    c.bench_function("rook magic", |b| {
        b.iter(|| {
            rook_moves_magic(
                black_box(Square::A1),
                black_box(BitBoard::from_bits(0x123123123)),
            )
        });
    });
    c.bench_function("rook magic unsafe", |b| {
        b.iter(|| {
            rook_moves_magic_unsafe(
                black_box(Square::A1),
                black_box(BitBoard::from_bits(0x123123123)),
            )
        });
    });
}

criterion_group!(benches, bishop, rook);
criterion_main!(benches);
