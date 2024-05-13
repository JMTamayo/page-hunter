# PAGE HUNTER

***Page Hunter*** library is a Rust-based pagination tool that provides a way to manage and navigate through pages of data.
It offers a set of resources that encapsulates all the necessary pagination information such as the current page, total pages, previous page, next page and the items on the current page.

The library also includes validation methods to ensure the integrity of the pagination data.
It's designed to be flexible and easy to integrate into any Rust project that requires pagination functionality.

To use **page-hunter** from GitHub repository with specific version, set the dependency in Cargo.toml file as follows:

```ini
[dependencies]
page-hunter = {git = "https://github.com/JMTamayo/page-hunter.git", version = "0.1.0", features = ["serde"] }
```

You can depend on it via cargo by adding the following dependency to your `Cargo.toml` file:

```ini
[dependencies]
page-hunter = { version = "0.1.0", features = ["serde"] }
```

## CRATE FEATURES
- `serde`: Add [Serialize](https://docs.rs/serde/1.0.197/serde/trait.Serialize.html) and [Deserialize](https://docs.rs/serde/1.0.197/serde/trait.Deserialize.html) support for `Page` based on crate [serde](https://crates.io/crates/serde/1.0.197). This feature is useful for implementing pagination models as a request or response body in REST APIs, among other implementations.

## BASIC OPERATION
To create a `Page` instance from known parameters:
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

If you need to paginate records and get a specific page:
```rust,no_run
    use page_hunter::*;

    let records: Vec<u32> = vec![1, 2, 3, 4, 5];
    let page: usize = 0;
    let size: usize = 2;

    let pagination_result: PaginationResult<Page<u32>> =
        paginate_records(&records, page, size);
```

On feature `serde` enabled, you can serialize and deserialize `Page` instances as follows:
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

When you create a new `Page` instance from a JSON string (deserialization), you can verify the integrity of the data with the `validate_fields()` method:

## CONTRIBUTIONS
The ***Page Hunter*** project is open source and therefore any interested software developer can contribute to its improvement. To contribute, take a look at the following recommendations:

- **Bug Reports**: If you find a bug, please create an issue detailing the problem, the steps to reproduce it, and the expected behavior.
- **Feature Requests**: If you have an idea for a new feature or an enhancement to an existing one, please create an issue describing your idea.
- **Pull Requests**: If you've fixed a bug or implemented a new feature, we'd love to see your work! Please submit a pull request. Make sure your code follows the existing style and all tests pass.
