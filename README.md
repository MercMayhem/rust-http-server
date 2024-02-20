# Merc's HTTP Web Server
This is my implementation of a multithreaded web server made using the Rust programming language. It listens to port 80 of localhost and is capable of serving static files as well as displaying `index.html` file present within a directory. It uses a thread pooling mechanism I built from scratch for fast and efficient handling of requests as the number of requests increase.

## Installation
1. Clone this repository locally: 
	`git clone https://github.com/MercMayhem/rust-http-server.git`
2. Install Rust and Cargo
3. Navigate to the source code and build it:
	`sudo cargo build`
	
## Run
Navigate to the directory where you want to run the server and run the executable present in the `target/debug/` directory:

`sudo /path/to/repo/target/debug/rust-http-server`

The web server will now be available to use on the `localhost:80`
