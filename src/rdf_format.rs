use std::str::FromStr;

use oxrdfio::RdfFormat as OxRdfFormat;

use crate::error::ZarrDfError;

pub enum RdfFormat {
    NTriples,
    Turtle,
    RdfXml,
    TriG,
    N3,
    NQuads,
}

impl RdfFormat {
    pub fn from_path(path: &str) -> Result<Self, ZarrDfError> {
        let extension = match path.split('.').last() {
            Some(extension) => extension,
            None => return Err(ZarrDfError::UnknownFormat(path.to_string())),
        };
        RdfFormat::from_str(extension)
    }
}

impl FromStr for RdfFormat {
    type Err = ZarrDfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nt" => Ok(RdfFormat::NTriples),
            "ttl" => Ok(RdfFormat::Turtle),
            "rdf" => Ok(RdfFormat::RdfXml),
            "trig" => Ok(RdfFormat::TriG),
            "n3" => Ok(RdfFormat::N3),
            "nq" => Ok(RdfFormat::NQuads),
            _ => Err(ZarrDfError::UnknownFormat(s.to_string())),
        }
    }
}

impl From<RdfFormat> for OxRdfFormat {
    fn from(format: RdfFormat) -> Self {
        match format {
            RdfFormat::NTriples => OxRdfFormat::NTriples,
            RdfFormat::Turtle => OxRdfFormat::Turtle,
            RdfFormat::RdfXml => OxRdfFormat::RdfXml,
            RdfFormat::TriG => OxRdfFormat::TriG,
            RdfFormat::N3 => OxRdfFormat::N3,
            RdfFormat::NQuads => OxRdfFormat::NQuads,
        }
    }
}
