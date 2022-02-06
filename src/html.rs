use crate::uri::{Uri, UriBuilder};

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
        let href = clean_up_href(path);

        if href.starts_with('#') {
            continue;
        }
        if href.starts_with("mailto") {
            continue;
        }

        let mut href_uri: UriBuilder = match href.parse::<Uri>() {
            Ok(uri) => UriBuilder::from(uri
            ),
            Err(e) => {
                errors.push(format!("{} in url {}", e, href));
                continue;
            }
        };
        if href_uri.uri.protocol.is_empty() {
            href_uri = href_uri.protocol(&parent_uri.protocol);
        }

        if href_uri.uri.host.is_empty() && href_uri.uri.path.starts_with("//") {
            let find = href_uri.uri.path[2..]
                .find('/')
                .map_or(href_uri.uri.path.len(), |f| f + 2);
            let path = String::from(&href_uri.uri.path);
            href_uri = href_uri.host(&path[2..(find - 2)]);
            href_uri = href_uri.path(&path[find..]);
        }
        if href_uri.uri.host.is_empty() || href_uri.uri.host == "." {
            href_uri = href_uri.host(&parent_uri.host);
            href_uri = href_uri.port(&parent_uri.port);
            let abs_path = absolutize_path(&href_uri.uri.path, &parent_uri.path);
            href_uri = href_uri.path(&abs_path);
        }

        href_uri = href_uri.fragment("");

        urls.push(href_uri.build());
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
    get_url_strings_from_doc(&mut paths_to_follow, body_str);
    get_urls_from_page(urls, errors, &paths_to_follow, uri);
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

    let result = out
        .iter()
        .fold(String::new(), |res, &s| format!("{}/{}", res, s));

    result
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
