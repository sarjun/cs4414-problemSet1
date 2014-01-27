//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut count:   int = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {
					println(format!("Received connection from: [{:s}]", pn.to_str()));
					unsafe {
						count = count + 1;
						println!("Total number of requests received: {:d}", count)
					}
				},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            let mut components : ~[~str] = ~[];
            for tempString in request_str.split(' ') {
            	components.push(tempString.to_owned());
            }
            if !str::eq(&components[1], &~"/") {
		let mut dotdot = false;
		let dot : u8 = ".".as_bytes()[0];
		let slash : u8 = "/".as_bytes()[0];
		for i in range (0, components[1].len()-3) {
			if(components[1][i]==dot && components[1][i+1]==dot && components[1][i+2]==slash) {
				dotdot = true;
			}
		}
            	if components[1].len() > 4 && !dotdot && str::eq(&components[1].slice_from(components[1].len()-5).to_owned(), &~".html") {
        	let f1name = components[1].slice_from(1);
        	let path1 = Path::new(f1name.clone());
        	let mut msg_file1 = File::open(&path1);
                let msg_bytes1: ~[u8] = msg_file1.read_to_end();
		stream.write(msg_bytes1);
		}
		else {
            let response: ~str = 
                 ~"HTTP/1.1 403 Forbidden\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>403 Forbidden</title>
                 <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 </style></head>
                 <body>
                 <h1>403: Forbidden</h1><h3>You do not have permission to access that file";
		stream.write(response.as_bytes());
		}
            }
            else {
            unsafe {
            let response: ~str = 
                 ("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>Hello, Rust!</title>
                 <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 </style></head>
                 <body>
                 <h1>Greetings, Krusty!</h1><h3>" + format!("Total Number of Requests: {:d}</h3>", count) +
                 "</body></html>\r\n");
            stream.write(response.as_bytes());
            }
            println!("Connection terminates.");
            }
        }
    }
}
