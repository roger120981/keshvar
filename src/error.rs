use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum SearchError {
    #[error("Could not be found in {searched_items}")]
    NotFound { searched_items: SearchedItems },
    #[error("There is no {0} to search")]
    NoItemToSearch(&'static str),
    #[error("Expected {expected}")]
    BadInput { expected: &'static str },
}

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum SearchedItems {
    #[error("all continents")]
    AllContinents,
    #[error("all regions")]
    AllRegions,
    #[error("all sub-regions")]
    AllSubRegions,
    #[error("all world regions")]
    AllWorldRegions,
    #[error("all countries")]
    AllCountries,
    #[error("{0} supported countries")]
    SupportedCountries(usize),
    #[cfg(feature = "subdivisions")]
    #[cfg_attr(feature = "subdivisions", error("all subdivision types"))]
    AllSubDivisionTypes,
    #[cfg(feature = "subdivisions")]
    #[cfg_attr(feature = "subdivisions", error("all {0} subdivisions"))]
    AllSubdivisions(usize),
}