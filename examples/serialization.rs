use zarrdf::storage::Storage;
use zarrs::filesystem::FilesystemStore;

fn main() -> Result<(), zarrdf::error::ZarrDfError> {
    let args: Vec<_> = std::env::args().collect();
    let rdf = args.get(1).expect("missing rdf file");
    let zarr = args.get(2).expect("missing zarr file");
    let store = FilesystemStore::new(zarr)?;
    Storage::from_rdf_and_store(rdf, store)?;
    Ok(())
}
