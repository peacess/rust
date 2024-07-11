//!Just doc sample for the mod rust::doc_
//!
//!
//! # Samples
//! ```
//! use rust::doc_::Data;
//!
//! let mod_data = Data::new_data();
//! ```
//!

/// Just doc sample for the rust::doc_::Data
///
///
/// # Samples
///
/// ```rust,editable
/// use rust::doc_::Data;
/// let data = Data::new_data();
/// ```
/// use the [`Data::new_data`]
///
/// use 2 the [rust::doc_::Data]
///
/// see the [`Vec`]
///
/// see1 the [Vec]
///
/// see2 the [std::vec::Vec]
///
pub struct Data {
    /// data's name
    /// see [Data::new]
    pub name: String,
}

impl Data {
    /// new struct [Data]
    /// see the field [Data::name]
    pub fn new_data() -> Self {
        Data { name: String::new() }
    }
}
