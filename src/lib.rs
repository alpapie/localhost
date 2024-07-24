use std::fs;
use std::path::Path;

pub fn is_directory(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}


  // pub fn response_200(&mut self, route: RouteConfig, path: String) -> Option<String> {
    //     if route.directory_listing {
    //         if let Some(default)  =route.default_file  {
    //             let p = format!("{}{}/{}", &route.root_directory, path,default);
    //             if route.cgi.is_some() {
    //                 let cgi_handler = CGIHandler::new(p);
    //                 if let Some(res) = cgi_handler.handle_request() {
    //                     self.header.push(format!("{} {}", "Content-Length:", res.len()+1));
    //                     self.header.push(format!("{} {}", "\r\n".to_owned(), res));
    //                 } else {
    //                     return None;
    //                 }
    //             } else {
    //                 match self.parse_page(&p) {
    //                     Some(content) => {
    //                         self.header
    //                             .push(format!("{} {}", "Content-Length:", content.len() +1));
    //                         self.header
    //                             .push(format!("{} {}", "\r\n".to_owned(), content));
    //                     }
    //                     None => return None,
    //                 }
    //             }
    //         }
    //         match self.list_directory(format!("{}{}", &route.root_directory, path)) {
    //             Some(content) => {
    //                 self.header.push(format!(
    //                     "{} {}",
    //                     "Content-Length:",
    //                     content.len() +1
    //                 ));
    //                 self.header
    //                     .push(format!("{} {}", "\r\n".to_owned(), content));
    //             }
    //             None => return None,
    //         }
    //     } else if !route.directory_listing {
    //         if route.cgi.is_some() {
    //             let p = format!("{}{}", route.root_directory, path);
    //             let cgi_handler = CGIHandler::new(p);
    //             if let Some(res) = cgi_handler.handle_request() {
    //                 self.header.push(format!("{} {}", "Content-Length:", res.len()+1));
    //                 self.header.push(format!("{} {}", "\r\n".to_owned(), res));
    //             } else {
    //                 return None;
    //             }
    //         } else {
    //             match self.parse_page(&(route.root_directory + &path)) {
    //                 Some(content) => {
    //                     self.header
    //                         .push(format!("{} {}", "Content-Length:", content.len() +1));
    //                     self.header
    //                         .push(format!("{} {}", "\r\n".to_owned(), content));
    //                 }
    //                 None => return None,
    //             }
    //         }
    //     }

    //     Some(self.format_header())
    // }