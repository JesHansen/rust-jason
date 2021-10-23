use criterion::{criterion_group, criterion_main, Criterion};
use jason::kanulja;
use jason::util::{deserialize_residents, map_residents, read_stringdata};

fn jasonparsing(c: &mut Criterion) {
    let json_text = read_stringdata("LillyAndI.json");
    let deserialized_data = deserialize_residents(&json_text);

    c.bench_function("read file", |b| {
        b.iter(|| read_stringdata("LillyAndI.json"))
    });

    c.bench_function("deserialize json", |b| {
        b.iter(|| deserialize_residents(&json_text))
    });
    c.bench_function("map to int", |b| {
        b.iter(|| map_residents(&deserialized_data))
    });
    c.bench_function("json complete parse", |b| b.iter(|| kanulja()));
}

criterion_group!(benches, jasonparsing);
criterion_main!(benches);
