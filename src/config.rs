use zarrs::array::ChunkGrid;
use zarrs::array::DataType;
use zarrs::array::FillValue;

use crate::index::Index;

pub struct Config {
    pub shape: Vec<u64>,
    pub data_type: DataType,
    pub chunk_grid: ChunkGrid,
    pub fill_value: FillValue,
    pub index: Index,
}
