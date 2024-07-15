use std::fs;
use std::path::Path;

use std::{collections::HashMap};

use crate::config::config::RouteConfig;


#[derive(Debug,Default,Clone)]
pub struct Response{
    pub status: u32,
    pub header:  HashMap<String,String>,
    pub content: String,
    pub path: String
}

impl Response {
    pub fn new(path: String)->Self{
        let mut header= HashMap::new();
        header.insert("HTTP/1.1".to_owned(), "200 OK".to_owned());
        header.insert("Content-Type:".to_owned(), "text/html".to_owned());
        header.insert("Content-Type:".to_owned(), "text/html".to_owned());
        return Self { status: 200,
            header, 
            content: "".to_owned() ,
            path
        }
    }

    // pub fn send_response(&mut self, )

    pub fn response_200(&mut self,route: RouteConfig)->String{
        if route.directory_listing{
          match self.list_directory(format!("{}{}",route.root_directory,self.path)) {
            Some(content) => {
                self.header.insert("Content-Length:".to_owned(), content);

            },
            None => return self.response_error(403),
            }  
        }
        
        let formatted_entries: Vec<String> = self.header
        .iter()
        .map(|(k, v)| format!("{} {}", k, v))
        .collect();
        formatted_entries.join("\r\n")
    }

    pub fn response_error(&mut self, status:u16)->String{
        "".to_owned()
    }

    pub fn parse_page(&mut self,route: &str)->String{
        "".to_owned()
    }
    
    fn list_directory(&mut self, path_t: String) ->Option<String>{
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
}
