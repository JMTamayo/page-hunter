use std::fmt::{Debug, Display};

#[cfg(feature = "serde")]
use serde::{
    de::{Deserialize as DeDeserialize, Deserializer as DeDeserializer},
    Deserialize, Serialize, Serializer,
};
#[cfg(feature = "utoipa")]
use utoipa::{
    openapi::{schema::Schema, ArrayBuilder, ObjectBuilder},
    ToSchema,
};

#[allow(unused_imports)]
use crate::{Page, PaginationError};

/// Model to represent a book of paginated items.
/// #### Fields:
/// - **sheets**: Represents the ***sheets*** in a [`Book`] as a [`Vec`]  of [`Page`].
pub struct Book<E> {
    sheets: Vec<Page<E>>,
}

impl<E> Book<E> {
    /// Get ***sheets***
    pub fn get_sheets(&self) -> &Vec<Page<E>> {
        &self.sheets
    }

    /// Create a new [`Book`] instance.
    ///
    /// ### Arguments:
    /// - **sheets**: A reference to a [`Vec`] of  [`Page`], where `E` must implement [`Clone`].
    ///
    /// ### Returns:
    /// A [`Book`] if successful, otherwise a [`PaginationError`] is returned.
    ///
    /// ### Example:
    /// ```rust,no_run
    /// use page_hunter::*;
    ///
    /// let sheets: Vec<Page<u32>> = vec![
    ///     Page::new(&vec![1, 2], 0, 2, 5).unwrap_or_else(|error| {
    ///         panic!("Error creating page model: {:?}", error);
    ///     }),
    ///     Page::new(&vec![3, 4], 1, 2, 5).unwrap_or_else(|error| {
    ///         panic!("Error creating page model: {:?}", error);
    ///     }),
    /// ];
    ///
    /// let book: Book<u32> = Book::new(&sheets);
    /// ```
    pub fn new(sheets: &Vec<Page<E>>) -> Book<E>
    where
        E: Clone,
    {
        Book {
            sheets: sheets.to_owned(),
        }
    }
}

/// Implementation of [`Clone`] for [`Book`].
impl<E> Clone for Book<E>
where
    E: Clone,
{
    fn clone(&self) -> Self {
        Book {
            sheets: self.sheets.to_owned(),
        }
    }
}

/// Implementation of [`Debug`] for [`Book`].
impl<E> Debug for Book<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Book {{ sheets: {:?} }}", self.sheets)
    }
}

/// Implementation of [`Default`] for [`Book`].
impl<E> Default for Book<E> {
    fn default() -> Self {
        Self { sheets: Vec::new() }
    }
}

/// Implementation of [`Display`] for [`Book`].
impl<E> Display for Book<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Book {{ sheets: {:?} }}", self.sheets)
    }
}

/// Implementation of [`IntoIterator`] for [`Book`].
impl<E> IntoIterator for Book<E> {
    type Item = Page<E>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.sheets.into_iter()
    }
}

/// Implementation of [`Serialize`] for [`Book`] if the feature `serde` is enabled.
#[cfg(feature = "serde")]
impl<E> Serialize for Book<E>
where
    E: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct BookModel<'a, E>
        where
            E: Serialize,
        {
            sheets: &'a Vec<Page<E>>,
        }

        let book_model: BookModel<E> = BookModel {
            sheets: &self.sheets,
        };

        book_model.serialize(serializer)
    }
}

/// Implementation of [`Deserialize`] for [`Book`] if the feature `serde` is enabled.
#[cfg(feature = "serde")]
impl<'de, E> DeDeserialize<'de> for Book<E>
where
    E: DeDeserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Book<E>, D::Error>
    where
        D: DeDeserializer<'de>,
    {
        #[derive(Deserialize)]
        struct BookModel<E> {
            sheets: Vec<Page<E>>,
        }

        let book_model: BookModel<E> = DeDeserialize::deserialize(deserializer)?;

        Ok(Book {
            sheets: book_model.sheets,
        })
    }
}

/// Implementation of [`ToSchema`] for [`Book`] if the feature `utoipa` is enabled.
#[cfg(feature = "utoipa")]
impl<'s, E> ToSchema<'s> for Book<E>
where
    E: ToSchema<'s>,
{
    fn schema() -> (&'s str, utoipa::openapi::RefOr<Schema>) {
        (
            "Book",
            ObjectBuilder::new()
                .description(Some("Model to represent a book of paginated items."))
                .property(
                    "sheets",
                    ArrayBuilder::new()
                        .description(Some(
                            "Represents a paginated items as a collection of pages",
                        ))
                        .items(Page::<E>::schema().1),
                )
                .required("sheets")
                .into(),
        )
    }
}
