use std::{fmt::Display, str::FromStr};

use regex::Regex;

#[derive(Debug, Default)]
pub struct UriBuilder {
    pub uri: Uri,
}

impl UriBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> Uri {
        self.uri
    }

    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.uri.protocol = protocol.into();
        self
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.uri.host = host.into();
        self
    }

    pub fn port(mut self, port: impl Into<String>) -> Self {
        self.uri.port = port.into();
        self
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.uri.path = path.into();
        self
    }

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.uri.query = query.into();
        self
    }

    pub fn fragment(mut self, fragment: impl Into<String>) -> Self {
        self.uri.fragment = fragment.into();
        self
    }
}

impl From<Uri> for UriBuilder {
    fn from(uri: Uri) -> Self {
        Self { uri }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Uri {
    pub protocol: String,
    pub host: String,
    pub port: String,
    pub path: String,
    pub query: String,
    pub fragment: String,
}

impl Uri {
    pub fn builder() -> UriBuilder {
        UriBuilder::new()
    }
}

impl FromStr for Uri {
    type Err = String;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        if url.is_empty() {
            return Ok(Uri::default());
        }

        let mut protocol = String::default();
        let mut host = String::default();
        let mut port = String::default();

        let end = url.len();

        let query_start = url.find('?').unwrap_or(end);
        let fragment_start = url.find('#').unwrap_or(end);

        let mut path_start;

        if let Some(protocol_end) = url.find(':') {
            let prot = &url[protocol_end..];

            let host_start = if prot.len() > 3 && &prot[0..3] == "://" {
                protocol = String::from(&url[0..protocol_end]);
                protocol_end + 3 // skipping over the "://"
            } else {
                0 // no protocol
            };

            path_start = url[host_start..]
                .find('/')
                .map_or(query_start, |s| s + host_start);

            let mut host_end = url[host_start..path_start].find(':').unwrap_or(path_start);

            host = String::from(&url[host_start..host_end]);

            if host == "." {
                host = String::new();
                path_start += 1;
            }

            //port
            if (host_end != url.len()) && &url[host_end..host_end + 1] == ":" {
                host_end += 1;
                let port_end = path_start;
                port = String::from(&url[host_end..port_end]);
            }
        } else {
            path_start = 0;
        }

        let query_start = if query_start > fragment_start {
            fragment_start
        } else {
            query_start
        };

        // path
        let path = if path_start != url.len() {
            String::from(&url[path_start..query_start])
        } else {
            String::default()
        };

        // query
        let query = if query_start != url.len() && query_start != fragment_start {
            String::from(&url[(query_start + 1)..fragment_start])
        } else {
            String::default()
        };

        let fragment = if fragment_start != url.len() {
            String::from(&url[(fragment_start + 1)..])
        } else {
            String::default()
        };

        let k_section_regex = Regex::new(r"(^[a-zA-Z0-9\._~!$&'()*+,;=:\\/@\-]*$)").unwrap();

        if !k_section_regex.is_match(&host) {
            return Err(format!("Invalid character in host ({})", host));
        }
        if !k_section_regex.is_match(&path) {
            return Err(format!("Invalid character in path ({})", path));
        }
        if !k_section_regex.is_match(&query) {
            return Err(format!("Invalid character in query ({})", query));
        }
        if !k_section_regex.is_match(&fragment) {
            return Err(format!("Invalid character in fragment ({})", fragment));
        }

        let result = Uri {
            protocol,
            host,
            port,
            path,
            query,
            fragment,
        };
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

        assert_eq!(url.protocol, "https");
        assert_eq!(url.host, "www.forbes.com");
        assert_eq!(url.port, "");
        assert_eq!(url.path, "/sites/christophersteiner/2016/09/29/how-to-hire-better-engineers-ignore-school-degrees-and-past-projects/");
        assert_eq!(url.query, "");
        assert_eq!(url.fragment, "ceda3f8360bf");
    }
}
