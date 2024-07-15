# Rust-http-server

# Motivation
I self-studied Rust and was unsure about what side project to undertake during my freshman year.
Therefore, I decided to write an HTTP Server with Rust, allowing others to call the server and obtain the resources they need.

# Dependencies
* [tokio](https://crates.io/crates/tokio)
* [colored](https://crates.io/crates/colored)

(you can see more in `Cargo.toml`)

# Usage
1. Enter this command in this directory
```shell
$ cargo run
```
2. Open the browser and you can see `Hello, World!` on the page
3. Also, some HTTP message appeared in the console


# Workflow
1. **Starting the server**: The program first starts a server on your computer at port 3000. This is like opening a shop and waiting for customers.
2. **Waiting for connections**: The program then waits for someone to connect to the server. This is like the shop waiting for customers to walk in.
3. **Handling connections**: When someone connects to the server, the program starts a new task to handle this connection. This is like a shop assistant helping a customer.
4. **Reading the request**: The program reads the data sent by the client and turns it into an HTTP request. This is like the customer telling the shop assistant what they want.
5. **Parsing the request**: The program looks at the HTTP request and figures out what the client wants. It then prints this information to the console. This is like the shop assistant understanding the customer’s order.
6. **Sending the response**: Finally, the program creates an HTTP response and sends it back to the client. This is like the shop assistant giving the customer what they asked for.

