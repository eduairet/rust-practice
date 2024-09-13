use web_programming::{extract_links_from_website, find_broken_links};

#[cfg(test)]
mod tests_extracting_links {
    use super::*;

    #[tokio::test]
    async fn test_extract_links_from_website() {
        let links = extract_links_from_website("https://www.youtube.com/").await;
        println!("{:?}", &links[..5]);
        assert!(links.len() > 0);
    }

    #[tokio::test]
    async fn test_find_broken_links() {
        let links = find_broken_links("https://acme.com/").await;
        println!("{:?}", &links);
    }
}
