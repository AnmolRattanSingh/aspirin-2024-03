use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let proxy_addr = "127.0.0.1:8081"; // Proxy listens here
    let origin_addr = "127.0.0.1:8080"; // Origin

    let listener = TcpListener::bind(proxy_addr).expect("couldn't bind to proxy address");
    println!(
        "Proxy listening on {}, forwarding to {}",
        proxy_addr, origin_addr
    );

    for stream in listener.incoming() {
        match stream {
            Ok(mut client_stream) => match TcpStream::connect(origin_addr) {
                Ok(mut server_stream) => {
                    if let Err(e) = handle_client(&mut client_stream, &mut server_stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Couldn't connect to origin server: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Couldn't accept client: {}", e);
            }
        }
    }
}

fn handle_client<RW1, RW2>(client_stream: &mut RW1, server_stream: &mut RW2) -> std::io::Result<()>
where
    RW1: Read + Write,
    RW2: Read + Write,
{
    // Read request from client
    let client_request = read_stream(client_stream)?;
    println!("Received {} bytes from client", client_request.len());

    // Forward request to server
    write_stream(server_stream, &client_request)?;
    println!("Forwarded request to server");

    // Read response from server
    let server_response = read_stream(server_stream)?;
    println!("Received {} bytes from server", server_response.len());

    // Forward response to client
    write_stream(client_stream, &server_response)?;
    println!("Sent response back to client");

    Ok(())
}

fn read_stream<R: Read>(stream: &mut R) -> std::io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut temp_buf = [0u8; 1024];

    loop {
        let bytes_read = stream.read(&mut temp_buf)?;
        if bytes_read == 0 {
            break; // Connection closed or no more data
        }
        buffer.extend_from_slice(&temp_buf[..bytes_read]);

        if bytes_read < temp_buf.len() {
            break;
        }
    }

    Ok(buffer)
}

fn write_stream<W: Write>(stream: &mut W, data: &[u8]) -> std::io::Result<()> {
    stream.write_all(data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_stream() {
        let data = b"Test data for read_stream";
        let mut cursor = Cursor::new(data);

        let result = read_stream(&mut cursor).expect("Failed to read from stream");

        assert_eq!(result, data);
    }

    #[test]
    fn test_write_stream() {
        let data = b"Test data for write_stream";
        let mut cursor = Cursor::new(Vec::new());

        write_stream(&mut cursor, data).expect("Failed to write to stream");

        assert_eq!(cursor.get_ref().as_slice(), data);
    }

    #[test]
    fn test_handle_client() {
        let client_data = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let server_data = b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";

        // Simulate client stream
        let mut client_read_cursor = Cursor::new(client_data.to_vec());
        let mut client_write_buffer = Vec::new();

        // Simulate server stream
        let mut server_read_cursor = Cursor::new(server_data.to_vec());
        let mut server_write_buffer = Vec::new();

        {
            // Create client and server streams using CursorStream
            let mut client_stream =
                CursorStream::new(&mut client_read_cursor, &mut client_write_buffer);
            let mut server_stream =
                CursorStream::new(&mut server_read_cursor, &mut server_write_buffer);

            handle_client(&mut client_stream, &mut server_stream).expect("handle_client failed");
        }
        assert_eq!(server_write_buffer, client_data);
        assert_eq!(client_write_buffer, server_data);
    }

    /// Simulate a stream that can both read and write using Cursors.
    struct CursorStream<'a> {
        read_cursor: &'a mut Cursor<Vec<u8>>,
        write_buffer: &'a mut Vec<u8>,
    }

    impl<'a> CursorStream<'a> {
        fn new(read_cursor: &'a mut Cursor<Vec<u8>>, write_buffer: &'a mut Vec<u8>) -> Self {
            Self {
                read_cursor,
                write_buffer,
            }
        }
    }

    impl<'a> Read for CursorStream<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.read_cursor.read(buf)
        }
    }

    impl<'a> Write for CursorStream<'a> {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.write_buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}
