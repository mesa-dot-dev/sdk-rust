//! Automatic cursor-based pagination via async streams.

use futures_core::Stream;

/// A response from a paginated API endpoint.
///
/// Implement this for each generated `200Response` type that carries
/// `next_cursor` / `has_more` fields and a vector of items.
pub(crate) trait PaginatedResponse {
    /// The individual item type yielded by the stream.
    type Item;

    /// Consume the response and return its items.
    fn items(self) -> Vec<Self::Item>;

    /// The cursor for the next page, if any.
    fn next_cursor(&self) -> Option<&str>;

    /// Whether the server reports more pages.
    fn has_more(&self) -> bool;
}

/// Drive cursor-based pagination, yielding individual items as a stream.
///
/// `fetch` is called with the current cursor (`None` for the first page)
/// and the page-size limit. The stream yields each item from every page
/// until the server signals there are no more results.
pub(crate) fn paginate<T, E, F, Fut>(
    limit: Option<u8>,
    fetch: F,
) -> impl Stream<Item = Result<T::Item, E>>
where
    T: PaginatedResponse,
    F: Fn(Option<String>, Option<u8>) -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    async_stream::try_stream! {
        let mut cursor: Option<String> = None;
        loop {
            tracing::trace!(cursor = cursor.as_deref(), "fetching page");
            let page = fetch(cursor, limit).await?;
            let has_more = page.has_more();
            let next = page.next_cursor().map(String::from);
            tracing::trace!(has_more, next_cursor = next.as_deref(), "page received");
            for item in page.items() {
                yield item;
            }
            if !has_more {
                break;
            }
            cursor = next;
            if cursor.is_none() {
                break;
            }
        }
    }
}
