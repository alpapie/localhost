use std::error::Error;
use std::fs;
use std::path::Path;

use localhost::is_directory;

use crate::cgi::CGIHandler;
use crate::config::config::RouteConfig;
use crate::config::Config;
use crate::error::LogError;

use super::HttpStatus;

#[derive(Debug, Default, Clone)]
pub struct Response {
    pub status: u32,
    pub header: Vec<String>,
    pub content: String,
    pub session: String
}

impl Response {
    pub fn new(session:String) -> Self {
        let header = vec!["HTTP/1.1 200 OK".to_owned(),"Content-Type: text/html".to_owned()];

        Self {
            status: 200,
            header,
            content: "".to_owned(),
            session
        }
    }

    pub fn response_200(&mut self, route: RouteConfig, path: String,method:String) -> Option<String> {
        if method.to_lowercase()=="delete"{
            match Response::delete_file(&format!("{}{}", route.root_directory, path)) {
                Ok(_) => {
                    self.add_content_length_header(self.content_suces().len());
                    self.header.push(format!("{} {}", "\r\n".to_owned(), self.content_suces()));
                    return Some(self.format_header());
                } ,
                Err(_) => {
                    self.header[0] = format!(
                        "{} {}",
                        "HTTP/1.1".to_owned(),
                        HttpStatus::from_code(404)
                    );
                    let errorp=self.content_error(404);
                    self.add_content_length_header(errorp.len());
                    self.header.push(format!("{} {}", "\r\n".to_owned(), errorp));
                    return Some(self.format_header());
                } ,
            }
        }
        if route.directory_listing {
            self.handle_directory_listing(&route, &path)
        } else {
            self.handle_regular_request(&route, &path)
        }
    }

    fn handle_directory_listing(&mut self, route: &RouteConfig, path: &str) -> Option<String> {
        if let Some(default) = &route.default_file {
            let p = format!("{}{}/{}", route.root_directory, path, default);
            if let Some(content) = self.handle_cgi_or_page(route, &p) {
                return Some(content);
            }
            return None
        }

        if let Some(content) = self.list_directory(format!("{}{}", route.root_directory, path)) {
            self.add_content_length_header(content.len());
            self.header.push(format!("{} {}", "\r\n".to_owned(), content));
            return Some(self.format_header())
        }
        None
    }

    fn handle_regular_request(&mut self, route: &RouteConfig, path: &str) -> Option<String> {
        let p = format!("{}{}", route.root_directory, path);
        self.handle_cgi_or_page(route, &p)
    }

    fn handle_cgi_or_page(&mut self, route: &RouteConfig, path: &str) -> Option<String> {
        if route.cgi.is_some() {
            if let Some(res) = self.handle_cgi_request(path) {
                self.add_content_length_header(res.len());
                self.header.push(format!("{} {}", "\r\n".to_owned(), res));
                return Some(self.format_header());
            }
        } else if let Some(content) = self.handle_page_request(path) {
            self.add_content_length_header(content.len());
            self.header.push(format!("{} {}", "\r\n".to_owned(), content));
            return Some(self.format_header());
        }
        None
    }

    fn handle_cgi_request(&self, path: &str) -> Option<String> {
        let cgi_handler = CGIHandler::new(path.to_string());
        cgi_handler.handle_request()
    }

    fn handle_page_request(&mut self, path: &str) -> Option<String> {
        self.parse_page(path)
    }

    fn add_content_length_header(&mut self, length: usize) {
        if !self.session.is_empty(){
            self.header.push(
               format!( "Set-Cookie: session_id={}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age={}",
            self.session,  24 * 60 * 60)
            );
        }
        self.header.push(format!("{} {}", "Content-Length:", length + 1));
    }



    // fn add_content_type_header(&mut self, typpe: String) {
    //     if self.header.len()>=2 {
    //         self.header[2]=format!("{} {}", "Content-Type: text/html", typpe);
    //     }
    // }

    fn format_header(&mut self) -> String {
        self.header.join("\r\n")
    }

    pub fn response_error(&mut self, status: u16, config: &Config) -> String {
        self.header[0] = format!(
            "{} {}",
            "HTTP/1.1".to_owned(),
            HttpStatus::from_code(status)
        );
        if let Some(page_error) = &config.error_pages {
            let path_page = match status {
                400 => &page_error.error_400,
                403 => &page_error.error_403,
                404 => &page_error.error_404,
                405 => &page_error.error_405,
                413 => &page_error.error_413,
                _ => &page_error.error_500,
            };
            if let Some(path) = path_page {
                if let Some(content) = self.parse_page(path) {
                    self.header.push(format!(
                        "{} {}",
                        "Content-Length:".to_owned(),
                        (content.len() + 1)
                    ));
                    self.header
                        .push(format!("{} {}", "\r\n".to_owned(), content));
                    return self.format_header();
                }
            }
        }
        let cont = self.content_error(status);
        self.header.push(format!(
            "{} {}",
            "Content-Length:".to_owned(),
            cont.len()+1
        ));
        self.header.push(format!("{} {}", "\r\n".to_owned(), cont));
        self.format_header()
    }

    pub fn parse_page(&mut self, route: &str) -> Option<String> {
        match fs::read_to_string(route) {
            Ok(content) => Some(content),
            Err(err) => {
                LogError::new(format!("parse eror page-> {}",err)).log();
                None
            },
        }
    }

    pub fn list_directory(&mut self, path_t: String) -> Option<String> {
        let mut response = String::new();
        let p = Path::new(&path_t);
        if !is_directory(p) {
            return None;
        }

        let paths_p = fs::read_dir(&path_t);
        response.push_str("<html><body><h1>Directory Listing</h1><ul>");
        if let Ok(paths) = paths_p {
            for path in paths {
                // println!("Name: {}", path.unwrap().path().display());
                match path {
                    Ok(entry) => {
                        if let Some(file_name_str) =
                            entry.path().display().to_string().strip_prefix(&path_t)
                        {
                            response.push_str(&format!(
                                "<li><a href=\"{}\">{}</a></li>",
                                file_name_str, file_name_str
                            ));
                        };
                    }
                    Err(_) => return None,
                }
            }
            response.push_str("</ul></body></html>\n");
            return Some(response);
        }
        None
    }

    fn content_error(&self, code: u16) -> String {
        let styles = r#"
            <style>
            * {
                transition: all 0.6s;
            }
    
            html {
                height: 100%;
            }
    
            body {
                font-family: 'Lato', sans-serif;
                color: #888;
                margin: 0;
            }
    
            #main {
                display: table;
                width: 100%;
                height: 100vh;
                text-align: center;
            }
    
            .fof {
                display: table-cell;
                vertical-align: middle;
            }
    
            .fof h1 {
                font-size: 50px;
                display: inline-block;
                padding-right: 12px;
                animation: type .5s alternate infinite;
            }
    
            @keyframes type {
                from { box-shadow: inset -3px 0px 0px #888; }
                to { box-shadow: inset -3px 0px 0px transparent; }
            }
            </style>
        "#;

        let html_start = r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Error Page</title>
        "#;

        let html_end = r#"
            </head>
            <body>
                <div id="main">
                    <div class="fof">
                        <h1>Error
        "#;

        let html_close = r#"
                        </h1>
                    </div>
                </div>
            </body>
            </html>
        "#;

        format!(
            "{}{}{} {}{}",
            html_start, styles, html_end, code, html_close
        )
    }

    fn content_suces(&self) -> String {
        let styles = r#"
            <style>
            * {
                transition: all 0.6s;
            }
    
            html {
                height: 100%;
            }
    
            body {
                font-family: 'Lato', sans-serif;
                color: #888;
                margin: 0;
            }
    
            #main {
                display: table;
                width: 100%;
                height: 100vh;
                text-align: center;
            }
    
            .fof {
                display: table-cell;
                vertical-align: middle;
            }
    
            .fof h1 {
                font-size: 50px;
                display: inline-block;
                padding-right: 12px;
                animation: type .5s alternate infinite;
            }
    
            @keyframes type {
                from { box-shadow: inset -3px 0px 0px #888; }
                to { box-shadow: inset -3px 0px 0px transparent; }
            }
            </style>
        "#;

        let html_start = r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Delete Page</title>
        "#;

        let html_end = r#"
            </head>
            <body>
                <div id="main">
                    <div class="fof">
                        <h1>Delete success
        "#;

        let html_close = r#"
                        </h1>
                    </div>
                </div>
            </body>
            </html>
        "#;

        format!(
            "{}{}{} {}",
            html_start, styles, html_end, html_close
        )
    }
    pub fn delete_file(file_path: &str) -> Result<(), Box<dyn Error>> {
        fs::remove_file(file_path)?;
        Ok(())
      }
}
