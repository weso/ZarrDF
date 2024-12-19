use zarrdf::query::BasicGraphPattern;
use zarrdf::storage::Storage;
use zarrs::filesystem::FilesystemStore;

const RDF: &str = "resources/input.ttl";
const OUTPUT: &str = "output.zarr";

fn main() -> Result<(), zarrdf::error::ZarrDfError> {
    let store = FilesystemStore::new(OUTPUT)?;
    let storage = Storage::from_rdf_and_store(RDF, store)?;
    let triples = storage.triples_matching(None, None, None)?;
    println!("{:?}", triples);
    Ok(())
}
