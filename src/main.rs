use std::
{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener,TcpStream},

    thread,
    time::Duration,

};

use hello_web::ThreadPool;

fn main() 
{
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2)
    {
        let stream = stream.unwrap();

        println!("Connection established!");

        //handle_connection(stream);//single thread

        // thread::spawn //multi thread
        // (|| {
        //         handle_connection(stream);
        //     }
        // );

        pool.execute
        (||
            {
                handle_connection(stream);
            }
        );
        
    }

    println!("Shutting down!");

}


#[deprecated]
fn handle_connection_old(mut stream: TcpStream)//the function was refactored to handle_connection bellow
{
    let buf_reader = BufReader::new(&mut stream);

    // let http_request:Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    //let response = "HTTP/1.1 200 OK\r\n\r\n";

    //println!("Request: {:#?}", http_request);

    let request_line = buf_reader.lines().next().unwrap().unwrap();


    if request_line == "GET / HTTP/1.1"//check the HTTP METHOD if this request is for /.
    {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("pages/hello.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
    else //otherwise,
    {
        //some other request
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("pages/404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\n\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();

    }

    
    
}


fn handle_connection(mut stream: TcpStream)//should handle for error instead of unwarp()
{
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1"
    {
        ("HTTP/1.1 200 OK", "pages/hello.html")
    }
    else
    {
        ("HTTP/1.1 404 NOT FOUND", "pages/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}


// fn handle_connection_multithread(mut stream:TcpStream)
// {
//     let buf_reader = BufReader::new(&mut stream);

//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     let (status_line, filename) = match  &request_line[..]
//     {
//         "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "pages/hello.html"),
//         "GET /sleep HTTP/1.1" =>
//         {
//             thread::sleep(Duration::from_secs(5));
//             ("HTTP/1.1 200 OK", "pages/hello.html")
//         },//simulated the slow queued http request.
//         _ => ("HTTP/1.1 404 NOT FOUND", "pages/404.html"),
//     };


// }