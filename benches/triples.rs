#![feature(custom_test_frameworks)]
#![test_runner(criterion::runner)]

use std::sync::Arc;

use criterion::black_box;
use criterion::Criterion;
use criterion_macro::criterion;
use zarrdf::error::ZarrDfError;
use zarrdf::query::BasicGraphPattern;
use zarrdf::storage::Storage;
use zarrs::filesystem::FilesystemStore;
use zarrs::storage::ReadableWritableStorageTraits;

mod perf;

const ZARR: &str = "uniprot.zarr";

fn custom() -> Criterion {
    Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100))
}

fn triples<S: ReadableWritableStorageTraits + 'static>(
    store: Arc<Storage<S>>,
) -> Result<(), ZarrDfError> {
    store.triples_matching(None, None, None)?;
    Ok(())
}

#[criterion(custom())]
fn bench(c: &mut Criterion) {
    // -- SETUP --

    let store = FilesystemStore::new(ZARR).unwrap();
    let storage = Arc::new(Storage::from_zarr(store).unwrap());

    // -- BENCH FUNCTION --

    c.bench_function("Triples iteration", |b| {
        b.iter(|| triples(black_box(storage.clone())))
    });
}
