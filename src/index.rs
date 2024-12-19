use zarrs::array::DimensionName;

use crate::Triple;

pub enum Index {
    Subject,
    Predicate,
    Object,
}

impl Index {
    pub fn triple_to_component(&self, triple: &Triple) -> u64 {
        match self {
            Index::Subject => triple[0],
            Index::Predicate => triple[1],
            Index::Object => triple[2],
        }
    }

    pub fn dimension_names(&self) -> impl Iterator<Item = DimensionName> {
        match self {
            Index::Subject => [Index::Predicate, Index::Object],
            Index::Predicate => [Index::Subject, Index::Object],
            Index::Object => [Index::Subject, Index::Predicate],
        }
        .into_iter()
        .map(|s| s.to_string().into())
    }
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Index::Subject => "subject",
                Index::Predicate => "predicate",
                Index::Object => "object",
            }
        )
    }
}
