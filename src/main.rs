fn main() {
    let listener= std::net::TcpListener::bind("127.0.0.1:8000").unwrap();
   for stream in listener.incoming().flatten(){
    
   }
}
