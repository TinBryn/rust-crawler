use std::{fmt::Display, str::FromStr};

use regex::Regex;

#[derive(Debug, Default)]
pub struct Uri {
    protocol: String,
    host: String,
    port: String,
    path: String,
    query: String,
    fragment: String,
}

impl Uri {
    pub fn protocol(&self) -> &str {
        &self.protocol
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> &str {
        &self.port
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn fragment(&self) -> &str {
        &self.fragment
    }
}

impl FromStr for Uri {
    type Err = String;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        let mut result = Uri::default();

        if url.is_empty() {
            return Ok(result);
        }

        let mut query_start = url.find('?').unwrap_or_else(|| url.len());
        let fragment_start = url.find('#').unwrap_or_else(|| url.len());

        let mut path_start;

        if let Some(protocol_end) = url.find(':') {
            let prot = &url[protocol_end..];

            let host_start = if prot.len() > 3 && &prot[0..3] == "://" {
                result.protocol = String::from(&url[0..protocol_end]);
                protocol_end + 3 // skipping over the "://"
            } else {
                0 // no protocol
            };

            path_start = url[host_start..]
                .find('/')
                .map_or(query_start, |s| s + host_start);

            let mut host_end = url[host_start..path_start].find(':').unwrap_or(path_start);

            result.host = String::from(&url[host_start..host_end]);

            if result.host == "." {
                result.host = String::new();
                path_start += 1;
            }

            //port
            if (host_end != url.len()) && &url[host_end..host_end + 1] == ":" {
                host_end += 1;
                let port_end = path_start;
                result.port = String::from(&url[host_end..port_end]);
            }
        } else {
            path_start = 0;
        }

        if query_start > fragment_start {
            query_start = fragment_start;
        }

        // path
        if path_start != url.len() {
            result.path = String::from(&url[path_start..query_start]);
        }

        // query
        if query_start != url.len() && query_start != fragment_start {
            result.query = String::from(&url[(query_start + 1)..fragment_start]);
        }

        if fragment_start != url.len() {
            result.fragment = String::from(&url[(fragment_start + 1)..]);
        }

        let k_section_regex = Regex::new(r"(^[a-zA-Z0-9\._~!$&'()*+,;=:\\/@\-]*$)").unwrap();

        if !k_section_regex.is_match(&result.host) {
            return Err(format!("Invalid character in host ({})", result.host));
        }
        if !k_section_regex.is_match(&result.path) {
            return Err(format!("Invalid character in path ({})", result.path));
        }
        if !k_section_regex.is_match(&result.query) {
            return Err(format!("Invalid character in query ({})", result.query));
        }
        if !k_section_regex.is_match(&result.fragment) {
            return Err(format!(
                "Invalid character in fragment ({})",
                result.fragment
            ));
        }

        Ok(result)
    }
}

impl Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}://{}", self.protocol, self.host)?;
        if !self.port.is_empty() {
            write!(f, ":{}", self.port)?;
        }
        write!(f, "{}", self.path)?;
        if !self.query.is_empty() {
            write!(f, "?{}", self.query)?;
        }
        if !self.fragment.is_empty() {
            write!(f, "#{}", self.fragment)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn already_passing() {
        const URL: &str = "https://www.forbes.com/sites/christophersteiner/2016/09/29/how-to-hire-better-engineers-ignore-school-degrees-and-past-projects/#ceda3f8360bf";

        let url: Uri = URL.parse().unwrap();

        dbg!(&url);

        assert_eq!(url.protocol(), "https");
        assert_eq!(url.host(), "www.forbes.com");
        assert_eq!(url.port(), "");
        assert_eq!(url.path(), "/sites/christophersteiner/2016/09/29/how-to-hire-better-engineers-ignore-school-degrees-and-past-projects/");
        assert_eq!(url.query(), "");
        assert_eq!(url.fragment(), "ceda3f8360bf");
    }
}
