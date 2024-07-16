use std::fs;
use std::path::Path;

use std::{collections::HashMap};

use crate::config::config::RouteConfig;
use crate::config::Config;

use super::HttpStatus;


#[derive(Debug,Default,Clone)]
pub struct Response{
    pub status: u32,
    pub header:  HashMap<String,String>,
    pub content: String,
}

impl Response {
    pub fn new()->Self{
        let mut header= HashMap::new();
        header.insert("HTTP/1.1".to_owned(), "200 OK".to_owned());
        header.insert("Content-Type:".to_owned(), "text/html".to_owned());
        return Self { status: 200,
            header, 
            content: "".to_owned() 
        }
    }

    // pub fn send_response(&mut self, )

    pub fn response_200(&mut self,route: RouteConfig,path: String)->Option<String>{
        if route.directory_listing{
          match self.list_directory(format!("{}{}",route.root_directory,path)) {
            Some(content) => {
                self.header.insert("Content-Length:".to_owned(), content.len().to_string());
                self.header.insert("\r\n".to_owned(), content.to_string());

            },
            None => return None,
            }  
        }
        
        Some(self.format_header())
    }

    fn format_header(&mut self) -> String {
        let formatted_entries: Vec<String> = self.header
        .iter()
        .map(|(k, v)| format!("{} {}", k, v))
        .collect();
        formatted_entries.join("\r\n")
    }
    
    pub fn response_error(&mut self, status:u16, config: &Config)->String{
        self.header.insert("HTTP/1.1".to_owned(), HttpStatus::from_code(status).to_string());
        if let Some( page_error) =&config.error_pages{
          let path_page= match status {
               400=>&page_error.error_400,
               403=>&page_error.error_403,
               404=>&page_error.error_404,
               405=>&page_error.error_405,
               413=>&page_error.error_413,
               _=>&page_error.error_500,
           };
           if let Some(path)= path_page{
            if let Some(content) = self.parse_page(&path){
                self.header.insert("".to_owned(), content);
                return self.format_header() ;
            }
           }
        }
        self.header.insert("\r\n".to_owned(), self.content_error(status));
        self.format_header()
    }

    pub fn parse_page(&mut self,route: &str)->Option<String>{
        match  fs::read_to_string(route){
            Ok(content) => Some(content),
            Err(_) => None,
        } 
    }
    
    pub fn list_directory(&mut self, path_t: String) ->Option<String>{
        let mut response = String::new();
        let paths_p = fs::read_dir(&path_t);
        response.push_str("<html><body><h1>Directory Listing</h1><ul>");
        if let Ok(paths)= paths_p{
            for path in paths {
                // println!("Name: {}", path.unwrap().path().display());
                match  path {
                    Ok(entry) => {
                        if let Some(file_name_str) = entry.path().display().to_string().strip_prefix(&path_t){
                            response.push_str(&format!("<li><a href=\"{}\">{}</a></li>", file_name_str, file_name_str));
                            response.push_str("</ul></body></html>");
                            return Some(response)
                        } ;
                    },
                    Err(_) => return None,
                }
            }
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
            html_start,
            styles,
            html_end,
            code,
            html_close
        )
    }
    
}
