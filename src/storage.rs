use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::vec;

use itertools::Itertools;
use oxrdfio::RdfParser;
use zarrs::array::chunk_grid::RectangularChunkGrid;
use zarrs::array::codec::GzipCodec;
use zarrs::array::Array;
use zarrs::array::ArrayBuilder;
use zarrs::array::ArrayIndices;
use zarrs::array::ChunkGrid;
use zarrs::array::DataType;
use zarrs::array::FillValue;
use zarrs::array_subset::ArraySubset;
use zarrs::group::Group;
use zarrs::group::GroupBuilder;
use zarrs::storage::ReadableWritableStorageTraits;

use crate::config::Config;
use crate::dictionary::Dictionary;
use crate::error::ZarrDfError;
use crate::index::Index;
use crate::rdf_format::RdfFormat;
use crate::utils::rdf_to_value;
use crate::utils::serialize_term;
use crate::utils::value_to_rdf;
use crate::Triple;

const CHUNK_SIZE: usize = 10000;

pub struct Storage<S: ReadableWritableStorageTraits> {
    pub dictionary: Dictionary,
    pub subject: Option<Array<S>>,
    pub predicate: Option<Array<S>>,
    pub object: Option<Array<S>>,
}

impl<S: ReadableWritableStorageTraits + 'static> Storage<S> {
    pub fn from_rdf_and_store(rdf_path: &str, store: S) -> Result<Self, ZarrDfError> {
        let input_file = File::open(rdf_path)?;
        let rdf_format = RdfFormat::from_path(rdf_path)?;
        let triples: Vec<_> = Self::read_rdf(input_file, rdf_format).collect();
        let store = Arc::new(store);

        let mut subjects = HashSet::new();
        let mut predicates = HashSet::new();
        let mut objects = HashSet::new();
        for triple in triples.iter() {
            subjects.insert(triple[0].clone());
            predicates.insert(triple[1].clone());
            objects.insert(triple[2].clone());
        }
        let dictionary = Dictionary::new(subjects, predicates, objects)?;

        let triples: Vec<_> = triples
            .into_iter()
            .filter_map(|[s, p, o]| {
                let triple = [
                    dictionary.get_subject_idx(&s)? as u64,
                    dictionary.get_predicate_idx(&p)? as u64,
                    dictionary.get_object_idx(&o)? as u64,
                ];
                Some(triple)
            })
            .collect();

        let mut group = GroupBuilder::new().build(store.clone(), "/")?;

        group.attributes_mut().insert(
            Index::Subject.to_string(),
            rdf_to_value(dictionary.subjects()),
        );
        group.attributes_mut().insert(
            Index::Predicate.to_string(),
            rdf_to_value(dictionary.predicates()),
        );
        group.attributes_mut().insert(
            Index::Object.to_string(),
            rdf_to_value(dictionary.objects()),
        );

        group.store_metadata()?;

        let storage = Self {
            dictionary,
            subject: Some(Self::write_index(
                triples.iter().cloned(),
                store.clone(),
                Index::Subject,
            )?),
            predicate: Some(Self::write_index(
                triples.iter().cloned(),
                store.clone(),
                Index::Predicate,
            )?),
            object: Some(Self::write_index(
                triples.into_iter(),
                store.clone(),
                Index::Object,
            )?),
        };

        Ok(storage)
    }

    pub fn from_zarr(store: S) -> Result<Self, ZarrDfError> {
        let store = Arc::new(store);
        let group = Group::open(store.clone(), "/")?;
        let attributes = group.attributes();

        let subjects = Array::open(store.clone(), &format!("/{}", Index::Subject))?;
        let predicates = Array::open(store.clone(), &format!("/{}", Index::Predicate))?;
        let objects = Array::open(store.clone(), &format!("/{}", Index::Object))?;

        let dictionary = Dictionary::new(
            match attributes.get(&Index::Subject.to_string()) {
                Some(subjects) => value_to_rdf(subjects),
                None => return Err(ZarrDfError::SubjectsNotInMetadata),
            },
            match attributes.get(&Index::Predicate.to_string()) {
                Some(predicates) => value_to_rdf(predicates),
                None => return Err(ZarrDfError::PredicatesNotInMetadata),
            },
            match attributes.get(&Index::Object.to_string()) {
                Some(objects) => value_to_rdf(objects),
                None => return Err(ZarrDfError::ObjectsNotInMetadata),
            },
        )?;

        let storage = Self {
            dictionary,
            subject: Some(subjects),
            predicate: Some(predicates),
            object: Some(objects),
        };

        Ok(storage)
    }

    fn read_rdf<R: Read>(read: R, format: RdfFormat) -> impl Iterator<Item = [String; 3]> {
        RdfParser::from_format(format.into())
            .for_reader(read)
            .flatten()
            .filter_map(move |quad| {
                let subject = serialize_term(quad.subject);
                let predicate = serialize_term(quad.predicate);
                let object = serialize_term(quad.object);

                if subject.is_empty() || predicate.is_empty() || object.is_empty() {
                    return None;
                } else {
                    Some([subject, predicate, object])
                }
            })
    }

    fn write_index(
        triples: impl Iterator<Item = Triple>,
        store: Arc<S>,
        index: Index,
    ) -> Result<Array<S>, ZarrDfError> {
        let chunked_triples = triples.chunk_by(|triple| index.triple_to_component(triple));

        let mut chunk_sizes = Vec::new();
        let mut triples = Vec::new();
        for (_, chunk) in chunked_triples.into_iter() {
            let chunk = chunk.map(|[a, b, _]| [a, b]).flatten().collect::<Vec<_>>();
            chunk_sizes.push(chunk.len() as u64 / 2);
            triples.push(chunk);
        }

        let config = Config {
            shape: vec![chunk_sizes.iter().sum(), 2],
            data_type: DataType::UInt64,
            chunk_grid: ChunkGrid::new(RectangularChunkGrid::new(&[
                chunk_sizes.try_into()?, // x dimension
                2.try_into()?,           // y dimension
            ])),
            fill_value: FillValue::from(0u64),
            index,
        };

        Self::serialize(triples.into_iter(), store, config)
    }

    fn serialize(
        triples: impl Iterator<Item = Vec<u64>>,
        store: Arc<S>,
        config: Config,
    ) -> Result<Array<S>, ZarrDfError> {
        let array = ArrayBuilder::new(
            config.shape,
            config.data_type,
            config.chunk_grid,
            config.fill_value,
        )
        .bytes_to_bytes_codecs(vec![Arc::new(GzipCodec::new(5)?)])
        .dimension_names(Some(config.index.dimension_names()))
        .build(store.clone(), &format!("/{}", config.index))?;

        array.store_metadata()?;

        // TODO: Parallelize
        for (i, chunk) in triples.chunks(CHUNK_SIZE).into_iter().enumerate() {
            let chunk: Vec<u64> = chunk.flatten().collect();
            let size = chunk.len() / 2;
            let start = ArrayIndices::from([(i * CHUNK_SIZE) as u64, 0]);
            let end = ArrayIndices::from([size as u64, 2]);
            let subset = ArraySubset::new_with_start_shape(start, end)?;
            array.store_array_subset_elements(&subset, &chunk[..])?;
        }

        Ok(array)
    }
}
