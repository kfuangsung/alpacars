//! News (`/v1beta1/news`), checked against the news-3 reference; fixture is its 200 example.

mod common;

use alpacars::data::historical::news::NewsRequest;
use wiremock::matchers::{method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Serve `first` (whose `next_page_token` is `token`) when no page_token is sent, and `last`
/// when `page_token == token`: the client must follow the token and merge both pages.
async fn mount_two_pages(server: &MockServer, p: &str, token: &str, first: &str, last: &str) {
    Mock::given(method("GET"))
        .and(path(p))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_string(first))
        .expect(1)
        .mount(server)
        .await;
    Mock::given(method("GET"))
        .and(path(p))
        .and(query_param("page_token", token))
        .respond_with(ResponseTemplate::new(200).set_body_string(last))
        .expect(1)
        .mount(server)
        .await;
}

#[tokio::test]
async fn test_get_news_paginates() {
    let server = MockServer::start().await;
    mount_two_pages(&server, "/v1beta1/news", "MTY0MDk0ODkyMzAwMDAwMDAwMHwyNDg0MzE3MQ==", r#"{
  "news": [
    {
      "author": "Charles Gross",
      "content": "<p>This headline-only article is meant to show you why a stock is moving, the most difficult aspect of stock trading....</p>",
      "created_at": "2021-12-31T11:08:42Z",
      "headline": "Apple Leader in Phone Sales in China for Second Straight Month in November With 23.6% Share, According to Market Research Data",
      "id": 24843171,
      "images": [],
      "source": "benzinga",
      "summary": "This headline-only article is meant to show you why a stock is moving, the most difficult aspect of stock trading",
      "symbols": [
        "AAPL"
      ],
      "updated_at": "2021-12-31T11:08:43Z",
      "url": "https://www.benzinga.com/news/21/12/24843171/apple-leader-in-phone-sales-in-china-for-second-straight-month-in-november-with-23-6-share-according"
    }
  ],
  "next_page_token": "MTY0MDk0ODkyMzAwMDAwMDAwMHwyNDg0MzE3MQ=="
}"#, r#"{
  "news": [
    {
      "author": "Charles Gross",
      "content": "<p>This headline-only article is meant to show you why a stock is moving, the most difficult aspect of stock trading....</p>",
      "created_at": "2021-12-31T11:08:42Z",
      "headline": "Apple Leader in Phone Sales in China for Second Straight Month in November With 23.6% Share, According to Market Research Data",
      "id": 24843171,
      "images": [],
      "source": "benzinga",
      "summary": "This headline-only article is meant to show you why a stock is moving, the most difficult aspect of stock trading",
      "symbols": [
        "AAPL"
      ],
      "updated_at": "2021-12-31T11:08:43Z",
      "url": "https://www.benzinga.com/news/21/12/24843171/apple-leader-in-phone-sales-in-china-for-second-straight-month-in-november-with-23-6-share-according"
    }
  ],
  "next_page_token": null
}"#).await;
    let client = common::news_client(&server.uri());
    let req = NewsRequest {
        symbols: Some("AAPL".to_string()),
        ..Default::default()
    };
    let news = client.get_news(&req).await.unwrap();
    assert_eq!(news.len(), 2);
}
