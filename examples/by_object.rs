use zarrdf::query::BasicGraphPattern;
use zarrdf::storage::Storage;
use zarrs::filesystem::FilesystemStore;

const RDF: &str = "resources/input.ttl";
const OUTPUT: &str = "output.zarr";
const OBJECT: &str = "\"1990-05-02\"^^<http://www.w3.org/2001/XMLSchema#date>";

fn main() -> Result<(), zarrdf::error::ZarrDfError> {
    let store = FilesystemStore::new(OUTPUT)?;
    let storage = Storage::from_rdf_and_store(RDF, store)?;
    let object = storage.dictionary.get_object_idx(OBJECT);
    let triples = storage.triples_matching(None, None, object)?;
    println!("{:?}", triples);
    Ok(())
}
