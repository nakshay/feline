/*MIT License

Copyright (c) 2020 Akshay Naik

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use futures::io;
use std::net::{TcpListener,TcpStream};
use smol::{Async,Task}; 


pub fn start_server() -> io::Result<()> {
    smol::run(async {
        
        let listener = Async::<TcpListener>::bind("127.0.0.1:6380")?;
        println!("feline started.... Listening on port {}", listener.get_ref().local_addr()?);
        
        loop {
            let (stream,peer_addr) = listener.accept().await?;
            println!("accepted connection from client {}", peer_addr);

            Task::spawn(handle_connection(stream)).unwrap().detach();
        };
    })
}

async fn handle_connection(stream : Async<TcpStream>) -> io::Result<()> {
    
    io::copy(&stream, &mut &stream).await?;
    Ok(())
}
