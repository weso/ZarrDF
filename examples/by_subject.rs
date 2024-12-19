use zarrdf::query::BasicGraphPattern;
use zarrdf::storage::Storage;
use zarrs::filesystem::FilesystemStore;

const RDF: &str = "resources/input.ttl";
const OUTPUT: &str = "output.zarr";
const SUBJECT: &str = "<http://example.org/a>";

fn main() -> Result<(), zarrdf::error::ZarrDfError> {
    let store = FilesystemStore::new(OUTPUT)?;
    let storage = Storage::from_rdf_and_store(RDF, store)?;
    let subject = storage.dictionary.get_subject_idx(SUBJECT);
    let triples = storage.triples_matching(subject, None, None)?;
    println!("{:?}", triples);
    Ok(())
}
