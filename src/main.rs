mod calc;
use calc::load_config;
use tokio::{net::{TcpListener, TcpStream},
            io::{AsyncWriteExt, AsyncBufReadExt, BufReader}};

//The responsibilities that remain in the main function 
// should be limited to the following:

// Calling the command line parsing logic with the argument values
// Setting up any other configuration
// Calling a run function in lib.rs
// Handling the error if run returns an error

//server logic should get moved out

#[tokio::main]
async fn main() {
    //plan:
    //parse and validate yaml
    //handle connection
    //return result

    let calc_conf = load_config().expect("Invalid YAML configuration");
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (stream, _) = listener
            .accept()
            .await
            .unwrap();
        println!("Connection Established");

        //spawning a tokio task for each connection
        tokio::spawn(async move { //'move' transfers ownership of stream into the spawned task
            handle_connection(stream).await
        });
    }
}

async fn handle_connection( mut stream: TcpStream) {

    //why is there a benefit of using tokios anychronous reader?
    let mut reader = BufReader::new(&mut stream);
    let mut http_req = Vec::new();
    let mut line = String::new();
    
    loop {
        line.clear(); 
        //why &mut line here?
        let bytes_read = reader.read_line(&mut line).await.expect("Failed to read line"); 
        // Stop reading when headers end (empty line) or client disconnects 
        if bytes_read == 0 || line == "\r\n" || line == "\n" { 
            break; 
        }

        http_req.push(line.trim().to_string()); //how can you trim before even converting to string

    } //why no ; here?

    println!("Request: {http_req:#?}"); //:#? is a pretty print debug format

    let req_header = match http_req.get(0) { //why not http_req[0]? 
        //BECAUSE `http_req.get(0)` safely returns `None` and http_req[0] would crash if there's nothing

        //okay rust doesn't have null, 
        //so there's either something or nothing wrapped inside Option<>
        Some(line) => line.as_str(), 
        None => "",
    }; 

    //Actual evaluation should be returned here
    //pretend response
    let result = serde_json::json!({"c" : 5});

    let (status_ln, body) = match req_header{
        "POST /calculate HTTP/1.1" => ("HTTP/1.1 200 OK", result.to_string()),
        _ => ("HTTP/1.1 404 NOT FOUND", tokio::fs::read_to_string("404.html").await.expect("failed to return 404 page"))
    };

    let length = body.len();

    let response = format!("{status_ln}\r\nContent-Length: {length}\r\n\r\n{body}");

    stream.write_all(response.as_bytes()).await.unwrap(); //okay, so in a normal world, there would be error handling 

}