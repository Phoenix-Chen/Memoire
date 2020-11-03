use std::fmt;

const TLDR_PAGES_URL: &str = "https://raw.githubusercontent.com/tldr-pages/tldr/master/";

pub async fn download_tldr(page_path: &str) -> Result<String, reqwest::Error> {
    Ok(reqwest::get(
        &format!("{}{}.md", TLDR_PAGES_URL, page_path)
    ).await?.text().await?)
}

pub fn parse_page(page_body: &str) -> Result<TLDRPage, TLDRPageParseError> {
    let mut tldr_page = TLDRPage::new();
    tldr_page.parse(page_body);
    Ok(tldr_page)
}

pub struct TLDRPage {
    command_name: Option<String>,
    examples: Vec<(String, String)>,
    expecting_command: bool
}

impl TLDRPage {
    pub fn new() -> TLDRPage {
        TLDRPage {
            command_name: None,
            examples: Vec::new(),
            expecting_command: false
        }
    }

    pub fn parse(&mut self, page_body: &str) -> Result<(), TLDRPageParseError> {
        for line in page_body.lines() {
            if let Err(tldr_page_parse_err) = self.parse_line(line) {
                return Err(tldr_page_parse_err);
            };
        }
        if self.expecting_command {
            return Err(TLDRPageParseError);
        }
        Ok(())
    }

    fn parse_line(&mut self, line: &str) -> Result<(), TLDRPageParseError> {
        // Ignore blank line
        let line_len = line.len();
        if line_len > 0 {
            match line.chars().next().unwrap() {
                '#' => {
                    match self.command_name {
                        Some(_) => {
                            return Err(TLDRPageParseError);
                        },
                        None => {
                            self.command_name = Some((&line[1..]).trim().to_owned());
                        }
                    }
                },
                '>' => {}, // Ignore short description
                '-' => {
                    if self.expecting_command {
                        return Err(TLDRPageParseError);
                    }
                    self.examples.push(
                        ((&line[1..]).trim().to_owned(), "".to_owned())
                    );
                    self.expecting_command = true;
                },
                '`' => {
                    if !self.expecting_command {
                        return Err(TLDRPageParseError);
                    }
                    let example_len = self.examples.len();
                    self.examples[example_len - 1].1 = (&line[..line_len - 1][1..]).trim().to_owned();
                    self.expecting_command = false;
                },
                _ => {
                    return Err(TLDRPageParseError);
                }
            }
        }
        Ok(())
    }

    pub fn get_examples(&self) -> &Vec<(String, String)> {
        &self.examples
    }
}

pub struct TLDRPageParseError;

impl fmt::Debug for TLDRPageParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl fmt::Display for TLDRPageParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to parse input tldr page, please check format.")
    }
}
