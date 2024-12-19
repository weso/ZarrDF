use zarrdf::query::BasicGraphPattern;
use zarrdf::storage::Storage;
use zarrs::filesystem::FilesystemStore;

const RDF: &str = "resources/input.ttl";
const OUTPUT: &str = "output.zarr";
const SUBJECT: &str = "<http://example.org/a>";
const PREDICATE: &str = "<http://example.org/birthdate>";
const OBJECT: &str = "\"1990-05-02\"^^<http://www.w3.org/2001/XMLSchema#date>";

#[test]
fn test_by_subject() -> Result<(), zarrdf::error::ZarrDfError> {
    let store = FilesystemStore::new(OUTPUT)?;
    let storage = Storage::from_rdf_and_store(RDF, store)?;
    let subject = storage.dictionary.get_subject_idx(SUBJECT);
    let triples = storage.triples_matching(subject, None, None)?;
    assert_eq!(triples.len(), 3);
    Ok(())
}

#[test]
fn test_by_predicate() -> Result<(), zarrdf::error::ZarrDfError> {
    let store = FilesystemStore::new(OUTPUT)?;
    let storage = Storage::from_rdf_and_store(RDF, store)?;
    let predicate = storage.dictionary.get_predicate_idx(PREDICATE);
    let triples = storage.triples_matching(None, predicate, None)?;
    assert_eq!(triples.len(), 1);
    Ok(())
}

#[test]
fn test_by_object() -> Result<(), zarrdf::error::ZarrDfError> {
    let store = FilesystemStore::new(OUTPUT)?;
    let storage = Storage::from_rdf_and_store(RDF, store)?;
    let object = storage.dictionary.get_object_idx(OBJECT);
    let triples = storage.triples_matching(None, None, object)?;
    assert_eq!(triples.len(), 1);
    Ok(())
}

#[test]
fn test_by_subject_predicate() -> Result<(), zarrdf::error::ZarrDfError> {
    let store = FilesystemStore::new(OUTPUT)?;
    let storage = Storage::from_rdf_and_store(RDF, store)?;
    let subject = storage.dictionary.get_subject_idx(SUBJECT);
    let predicate = storage.dictionary.get_predicate_idx(PREDICATE);
    let triples = storage.triples_matching(subject, predicate, None)?;
    assert_eq!(triples.len(), 1);
    Ok(())
}

#[test]
fn test_by_subject_object() -> Result<(), zarrdf::error::ZarrDfError> {
    let store = FilesystemStore::new(OUTPUT)?;
    let storage = Storage::from_rdf_and_store(RDF, store)?;
    let subject = storage.dictionary.get_subject_idx(SUBJECT);
    let object = storage.dictionary.get_object_idx(OBJECT);
    let triples = storage.triples_matching(subject, None, object)?;
    assert_eq!(triples.len(), 1);
    Ok(())
}

#[test]
fn test_by_predicate_object() -> Result<(), zarrdf::error::ZarrDfError> {
    let store = FilesystemStore::new(OUTPUT)?;
    let storage = Storage::from_rdf_and_store(RDF, store)?;
    let predicate = storage.dictionary.get_predicate_idx(PREDICATE);
    let object = storage.dictionary.get_object_idx(OBJECT);
    let triples = storage.triples_matching(None, predicate, object)?;
    assert_eq!(triples.len(), 1);
    Ok(())
}

#[test]
fn test_by_subject_predicate_object() -> Result<(), zarrdf::error::ZarrDfError> {
    let store = FilesystemStore::new(OUTPUT)?;
    let storage = Storage::from_rdf_and_store(RDF, store)?;
    let subject = storage.dictionary.get_subject_idx(SUBJECT);
    let predicate = storage.dictionary.get_predicate_idx(PREDICATE);
    let object = storage.dictionary.get_object_idx(OBJECT);
    let triples = storage.triples_matching(subject, predicate, object)?;
    assert_eq!(triples.len(), 1);
    Ok(())
}
