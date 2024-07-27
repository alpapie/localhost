use std::error::Error;
use std::fs;
use std::path::Path;

pub fn is_directory(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}


// pub fn read_event(&mut self) -> Result<(String, Vec<u8>), Error> {
//   // stream.set_nonblocking(true)?;
//   let mut reader = BufReader::new(&self.stream);
//   let mut request_line = String::new();

//   let mut head = String::new();
//   let mut body = Vec::new();
//   // Attempt to read the request line
//   loop {
//       match reader.read_line(&mut request_line) {
//           Ok(0) => {
//               return Err(Error::new(
//                   ErrorKind::WouldBlock,
//                   "No data available",
//               ))
//           }
//           Ok(_) => {
//               if !request_line.trim().is_empty() {
//                   break; // Successfully read the request line
//               }
//           }
//           Err(e) if e.kind() == ErrorKind::WouldBlock => continue, // Keep trying if no data is available yet
//           Err(e) => return Err(e),
//       }
//   }
//   // Read headers
//   let mut headers = Vec::new();
//   let mut header_line = String::new();
//   while reader.read_line(&mut header_line)? != 0 {
//       if header_line.trim().is_empty() {
//           break; // End of headers
//       }
//       headers.push(header_line.clone());
//       header_line.clear();
//   }
//   head= headers.join("\n");

//   // Read the body
//   let content_length=self.extract_content_length(&head).unwrap_or(0);
//   body.resize(content_length, 0);
//   reader.read_exact(&mut body)?;

//   Ok((head, body))
// }

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