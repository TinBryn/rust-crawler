use crate::uri::Uri;

pub struct HtmlHelper;

impl HtmlHelper {
    fn clean_up_href(href: &str) -> String {
        href.replace(" ", "%20")
    }
    fn get_urls_from_page(
        urls: &mut Vec<Uri>,
        errors: &mut Vec<String>,
        paths_to_follow: &[String],
        parent_uri: &Uri,
    ) {
        for path in paths_to_follow {
            let href = HtmlHelper::clean_up_href(path);

            if href.starts_with('#') {
                continue;
            }
            if href.starts_with("mailto") {
                continue;
            }

            let mut href_uri: Uri = match href.parse() {
                Ok(uri) => uri,
                Err(e) => {
                    errors.push(format!("{} in url {}", e, href));
                    continue;
                }
            };
            if href_uri.protocol().is_empty() {
                href_uri.set_protocol(parent_uri.protocol());
            }

            if href_uri.host().is_empty() && href_uri.path().starts_with("//") {
                let find = href_uri.path()[2..]
                    .find('/')
                    .map_or(href_uri.path().len(), |f| f + 2);
                let path = String::from(href_uri.path());
                href_uri.set_host(&path[2..(find - 2)]);
                href_uri.set_path(&path[find..]);
            }
            if href_uri.host().is_empty() || href_uri.host() == "." {
                href_uri.set_host(parent_uri.host());
                href_uri.set_port(parent_uri.port());
                href_uri.set_path(&HtmlHelper::absolutize_path(
                    href_uri.path(),
                    parent_uri.path(),
                ));
            }

            href_uri.set_fragment("");

            urls.push(href_uri);
        }
    }

    fn get_url_strings_from_doc(urls: &mut Vec<String>, body_str: &str) {
        let regexes = vec![
            regex::Regex::new("<a [^>]*href=\"([^\"]*)").unwrap(),
            regex::Regex::new("<link [^>]*href=\"([^\"]*)\"").unwrap(),
            regex::Regex::new("<script [^>]*src=\"([^\"]*)\"").unwrap(),
        ];

        for reg in regexes {
            for captures in reg.captures_iter(body_str) {
                urls.push(String::from(captures.get(1).unwrap().as_str()));
            }
        }
    }

    pub fn get_neighbors(urls: &mut Vec<Uri>, errors: &mut Vec<String>, body_str: &str, uri: &Uri) {
        let mut paths_to_follow = Vec::new();
        HtmlHelper::get_url_strings_from_doc(&mut paths_to_follow, body_str);
        HtmlHelper::get_urls_from_page(urls, errors, &paths_to_follow, uri);
    }

    pub fn absolutize_path(path: &str, base_path: &str) -> String {
        if path.is_empty() {
            return String::default();
        }

        if path.starts_with('/') && !path[1..].starts_with('.') {
            return String::from(path);
        }

        let mut sections = Vec::new();

        for section in base_path.split('/') {
            if !section.is_empty() {
                sections.push(section);
            }
        }

        if !base_path.ends_with('/') {
            sections.pop();
        }

        for section in path.split('/') {
            if !section.is_empty() {
                sections.push(section);
            }
        }

        let mut out = Vec::new();

        for section in sections {
            if section != "." && !section.is_empty() {
                if section != ".." {
                    out.push(section);
                } else {
                    out.pop();
                }
            }
        }

        let mut result = out
            .iter()
            .fold(String::new(), |res, &s| format!("{}/{}", res, s));

        result.push('/');

        result
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn regex_matches() {
        let reg = regex::Regex::new("<a [^>]*href=\"([^\"]*)").unwrap();

        for captures in reg.captures_iter("<a href=\"hello\"> <a href=\"world\"") {
            println!("{:?}", captures.get(1).unwrap().as_str());
        }
    }
}
