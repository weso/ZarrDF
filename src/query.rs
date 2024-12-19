use zarrs::array::Array;
use zarrs::array::ElementOwned;
use zarrs::storage::ReadableWritableStorageTraits;

use crate::error::ZarrDfError;
use crate::storage::Storage;

pub trait Query {
    fn query<T: ElementOwned>(&self, chunk: impl Into<u64>) -> Result<Vec<T>, ZarrDfError>;
}

impl<S: ReadableWritableStorageTraits + 'static> Query for Array<S> {
    fn query<T: ElementOwned>(&self, chunk: impl Into<u64>) -> Result<Vec<T>, ZarrDfError> {
        let chunk = chunk.into();
        self.retrieve_chunk_elements_if_exists(&[chunk, 0])?
            .ok_or(ZarrDfError::ChunkNotFound(chunk))
    }
}

pub trait BasicGraphPattern {
    type Term: ElementOwned;

    fn triples_matching(
        &self,
        subject: Option<i32>,
        predicate: Option<i32>,
        object: Option<i32>,
    ) -> Result<Vec<[Self::Term; 3]>, ZarrDfError>;
}

impl<S: ReadableWritableStorageTraits + 'static> BasicGraphPattern for Storage<S> {
    type Term = u64;

    fn triples_matching(
        &self,
        subject: Option<i32>,
        predicate: Option<i32>,
        object: Option<i32>,
    ) -> Result<Vec<[Self::Term; 3]>, ZarrDfError> {
        let subject = subject.map(|s| s as u64);
        let predicate = predicate.map(|p| p as u64);
        let object = object.map(|o| o as u64);

        let components: Vec<_> = match (subject, predicate, object) {
            (Some(subj), _, _) => self
                .subject
                .as_ref()
                .ok_or(ZarrDfError::InvalidQuery)?
                .query::<u64>(subj)?,
            (None, Some(pred), _) => self
                .predicate
                .as_ref()
                .ok_or(ZarrDfError::InvalidQuery)?
                .query::<u64>(pred)?,
            (None, None, Some(obj)) => self
                .object
                .as_ref()
                .ok_or(ZarrDfError::InvalidQuery)?
                .query::<u64>(obj)?,
            (None, None, None) => {
                let mut results = Vec::new();
                let mut i = 0;
                while let Some(chunk) = self
                    .subject
                    .as_ref()
                    .ok_or(ZarrDfError::InvalidQuery)?
                    .retrieve_chunk_elements_if_exists(&[i, 0])?
                {
                    results.extend(
                        chunk
                            .chunks(2)
                            .into_iter()
                            .map(|pair: &[u64]| [i, pair[0], pair[1]]),
                    );
                    i += 1;
                }
                return Ok(results);
            }
        };

        let triples = components
            .chunks(2)
            .into_iter()
            .filter_map(|pair: &[u64]| {
                let triple = match (subject, predicate, object) {
                    (Some(subject), _, _) => [subject, pair[0], pair[1]],
                    (None, Some(predicate), _) => [pair[0], predicate, pair[1]],
                    (None, None, Some(object)) => [pair[0], pair[1], object],
                    _ => return None, // this should never happen due to prior checks
                };
                if predicate.map_or(true, |pred| triple[1] == pred)
                    && object.map_or(true, |obj| triple[2] == obj)
                {
                    Some(triple)
                } else {
                    None
                }
            })
            .collect();

        Ok(triples)
    }
}
