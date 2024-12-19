use std::num::TryFromIntError;

use zarrs::array::codec::bytes_to_bytes::gzip::GzipCompressionLevelError;
use zarrs::array::ArrayCreateError;
use zarrs::array::ArrayError;
use zarrs::array_subset::IncompatibleDimensionalityError;
use zarrs::filesystem::FilesystemStoreCreateError;
use zarrs::group::GroupCreateError;
use zarrs::plugin::PluginCreateError;
use zarrs::storage::StorageError;

#[derive(thiserror::Error, Debug)]
pub enum ZarrDfError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unknown RDF format: {0}")]
    UnknownFormat(String),

    #[error("Chunk not found: {0}")]
    ChunkNotFound(u64),

    #[error("Invalid query")]
    InvalidQuery,

    #[error("Subjects not in metadata")]
    SubjectsNotInMetadata,

    #[error("Predicates not in metadata")]
    PredicatesNotInMetadata,

    #[error("Objects not in metadata")]
    ObjectsNotInMetadata,

    #[error(transparent)]
    RectangularDimensionConversion(#[from] TryFromIntError),

    #[error(transparent)]
    PathNotFound(#[from] GroupCreateError),

    #[error(transparent)]
    MetadataStorage(#[from] StorageError),

    #[error(transparent)]
    SettingCompressionLevel(#[from] GzipCompressionLevelError),

    #[error(transparent)]
    ArrayCreation(#[from] ArrayCreateError),

    #[error(transparent)]
    FilesystemCreation(#[from] FilesystemStoreCreateError),

    #[error(transparent)]
    ChunkGridConversion(#[from] PluginCreateError),

    #[error(transparent)]
    ChunkStorage(#[from] ArrayError),

    #[error(transparent)]
    DictionarySerialization(#[from] serde_json::Error),

    #[error(transparent)]
    DictionaryCreation(#[from] anyhow::Error),

    #[error(transparent)]
    ArraySubset(#[from] IncompatibleDimensionalityError),
}
