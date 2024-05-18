# PAGE HUNTER

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
[![dependency status](https://deps.rs/repo/github/JMTamayo/page-hunter/status.svg)](https://deps.rs/repo/github/JMTamayo/page-hunter)
[![CI](https://github.com/JMTamayo/page-hunter/actions/workflows/ci.yml/badge.svg)](https://github.com/JMTamayo/page-hunter/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/JMTamayo/page-hunter/graph/badge.svg?token=R1LAPNSV5J)](https://codecov.io/gh/JMTamayo/page-hunter)
[![crates.io](https://img.shields.io/crates/v/page-hunter.svg?label=crates.io&color=orange&logo=rust)](https://crates.io/crates/page-hunter)
[![docs.rs](https://img.shields.io/static/v1?label=docs.rs&message=page-hunter&color=blue&logo=docsdotrs)](http://docs.rs/page-hunter/latest/)


***Page Hunter*** library is a Rust-based pagination tool that provides a way to manage and navigate through pages of data.
It offers a set of resources that encapsulates all the necessary pagination information such as the current page, total pages, previous page, next page and the items on the current page.

The library also includes validation methods to ensure the integrity of the pagination data.
It's designed to be flexible and easy to integrate into any Rust project that requires pagination functionality and standard data validation.

To use **page-hunter** from GitHub repository with specific version, set the dependency in Cargo.toml file as follows:

```ini
[dependencies]
page-hunter = {git = "https://github.com/JMTamayo/page-hunter.git", version = "0.1.1", features = ["serde"] }
```

You can depend on it via cargo by adding the following dependency to your `Cargo.toml` file:

```ini
[dependencies]
page-hunter = { version = "0.1.1", features = ["serde"] }
```

## CRATE FEATURES
- `serde`: Add [Serialize](https://docs.rs/serde/1.0.197/serde/trait.Serialize.html) and [Deserialize](https://docs.rs/serde/1.0.197/serde/trait.Deserialize.html) support for `Page` and `Book` based on crate [serde](https://crates.io/crates/serde/1.0.197). This feature is useful for implementing pagination models as a request or response body in REST APIs, among other implementations.

## BASIC OPERATION

### Paginate records:

If you need to paginate records and get a specific `Page`:
```rust,no_run
    use page_hunter::*;

    let records: Vec<u32> = vec![1, 2, 3, 4, 5];
    let page: usize = 0;
    let size: usize = 2;

    let pagination_result: PaginationResult<Page<u32>> =
        paginate_records(&records, page, size);
```

To create a new instance of a `Page` from known parameters:
```rust,no_run
    use page_hunter::*;

    let items: Vec<u32> = vec![1, 2];
    let page: usize = 0;
    let size: usize = 2;
    let total_elements: usize = 5;

    let page_model_result: PaginationResult<Page<u32>> = Page::new(
        &items,
        page,
        size,
        total_elements,
    );
```

On feature `serde` enabled, you can serialize and deserialize a `Page` as follows:
```rust,no_run
    use page_hunter::*;

    let items: Vec<u32> = vec![1, 2];
    let page: usize = 0;
    let size: usize = 2;
    let total_elements: usize = 5;

    let page_model: PaginationResult<Page<u32>> = Page::new(
        &items,
        page,
        size,
        total_elements,
    ).unwrap();

    let serialized_page: String = serde_json::to_string(&page_model).unwrap();
    let deserialized_page: Page<u32> = serde_json::from_str(&serialized_page).unwrap();
```

When you create a new `Page` instance from the constructor or deserialization, the following rules are validated for the fields on the page:
- ***pages*** must be equal to ***total*** divided by ***size*** rounded up. When ***size*** is 0, ***pages*** must be 1.
- ***page*** must be less than or equal to ***pages*** - 1.
- if ***page*** is less than ***pages*** - 1, ***items*** length must be equal to ***size***.
- if ***page*** is equal to ***pages*** - 1, ***total*** must be equal to (***pages*** - 1) * ***size*** + ***items*** length.
- ***previous_page*** must be equal to ***page*** - 1 if ***page*** is greater than 0, otherwise it must be `None`.
- ***next_page*** must be equal to ***page*** + 1 if ***page*** is less than ***pages*** - 1, otherwise it must be `None`.

If any of these rules are violated, a `PaginationError` will be returned.

### Bind records:

If you need to bind records into a `Book` model:
```rust,no_run
    use page_hunter::*;

    let records: Vec<u32> = vec![1, 2, 3, 4, 5];
    let size: usize = 2;

    let book_result: PaginationResult<Book<u32>> =
        bind_records(&records, size);

    let book: Book<u32> = book_result.unwrap();
```

To create a new `Book` instance from known parameters:
```rust,no_run
    use page_hunter::*;

    let sheets: Vec<Page<u32>> = vec![
        Page::new(&vec![1, 2], 0, 2, 5).unwrap(),
        Page::new(&vec![3, 4], 1, 2, 5).unwrap(),
    ];

    let book: Book<u32> = Book::new(&sheets);
```

On feature `serde` enabled, you can serialize and deserialize a [`Book`] as follows:
```rust,no_run
    use page_hunter::*;

    let sheets: Vec<Page<u32>> = vec![
        Page::new(&vec![1, 2], 0, 2, 5).unwrap(),
        Page::new(&vec![3, 4], 1, 2, 5).unwrap(),
    ];

    let book: Book<u32> = Book::new(&sheets);

    let serialized_book: String = serde_json::to_string(&book).unwrap();
    let deserialized_book: Book<u32> = serde_json::from_str(&serialized_book).unwrap();
```

## CONTRIBUTIONS
The ***Page Hunter*** project is open source and therefore any interested software developer can contribute to its improvement. To contribute, take a look at the following recommendations:

- **Bug Reports**: If you find a bug, please create an issue detailing the problem, the steps to reproduce it, and the expected behavior.
- **Feature Requests**: If you have an idea for a new feature or an enhancement to an existing one, please create an issue describing your idea.
- **Pull Requests**: If you've fixed a bug or implemented a new feature, we'd love to see your work! Please submit a pull request. Make sure your code follows the existing style and all tests pass.
