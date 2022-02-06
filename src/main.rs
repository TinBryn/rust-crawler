use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    let _uri = uri::Uri::from_str("");
}

#[allow(unused)]
pub mod crawler;
pub mod graph;
pub mod html;
pub mod page_node;
pub mod uri;

#[cfg(test)]
mod test {

    mod already_passing_tests {

        use crate::{html, uri::Uri};

        #[test]
        fn crawl_page() {
            // let crawler = Crawler::with_max_threads(5);
            // let graph = crawler.crawl(
            //     "http://triplebyte.github.io/web-crawler-test-site/already-passing-tests/",
            //     true,
            // );

            // assert_eq!(graph["http://triplebyte.github.io/web-crawler-test-site/already-passing-tests/page2"].node_status, NodeStatus::Success);
            // assert_eq!(graph["http://triplebyte.github.io/web-crawler-test-site/already-passing-tests/page2-real"].node_status,  NodeStatus::Success);
            // assert_eq!(graph["http://triplebyte.github.io/web-crawler-test-site/already-passing-tests/page2-fake"].node_status, NodeStatus::Failure)
        }

        #[test]
        fn html_helper() {
            assert_eq!(
                html::absolutize_path("./page3", "/web-crawler-test-site/test4/cynical.html"),
                "/web-crawler-test-site/test4/page3"
            );
            assert_eq!(
                "SVG_logo.svg".parse::<Uri>().unwrap().path,
                "SVG_logo.svg"
            );
            assert_eq!(
                "./SVG_logo.svg".parse::<Uri>().unwrap().path,
                "./SVG_logo.svg"
            );
            assert_eq!(
                html::absolutize_path("./SVG_logo.svg", "/"),
                "/SVG_logo.svg"
            );
        }

        #[test]
        fn parsing_url() {
            let url1 = "https://www.forbes.com/sites/christophersteiner/2016/09/29/how-to-hire-better-engineers-ignore-school-degrees-and-past-projects/#ceda3f8360bf";
            let uri = url1.parse().unwrap();

            let expected = Uri::builder()
                .protocol("https")
                .host("www.forbes.com")
                .path("/sites/christophersteiner/2016/09/29/how-to-hire-better-engineers-ignore-school-degrees-and-past-projects/")
                .fragment("ceda3f8360bf")
                .build();

            assert_eq!(expected, uri);

            let s = uri.to_string();

            assert_eq!(s, "https://www.forbes.com/sites/christophersteiner/2016/09/29/how-to-hire-better-engineers-ignore-school-degrees-and-past-projects/#ceda3f8360bf"
        )
        }
    }
}
