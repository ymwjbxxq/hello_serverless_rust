# Hello Serverless Rust

When I speak about serverless, I always mention SPEED === MONEY.

I am one of the early adopters of the Lambda function, and at that time, the support for .NET was not available, so I started working with Node.js.

Nowadays, Node.js and Python dominate the market because (in my opinion) they were the only languages supported years ago.
![picture](https://github.com/ymwjbxxq/hello_serverless_rust/blob/main/1634456840046.gif)

There are many blogs out there that show that Node.js and Python are among the slowest languages, and I also have compared [.NET Core vs Node.js](https://github.com/ymwjbxxq?tab=repositories&q=dotnetcore3.1-vs-nodejs&type=&language=&sort=)

Lately, I got curious about Go and Rust's performance, and I decided to pick one and choose Rust.

### Mac Configuration ###

Let's install Rust:
```
brew install rustup
```
After this:
```
rustup-init
```
And select option 1, and once it is finished, you should be able to see the version. In my case
```
rustc -V
rustc 1.56.0 (09c42c458 2021-10-18)
```
I am using VS Code, and we need enabling Rust we need to install:
*  [Rust support for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
*  [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

Now from the terminal of VSCode
```
cargo --version
cargo 1.56.0 (4ed5d137b 2021-10-04)
```

Now we have installed all, and you can create your project with 
```
cargo new ServerlessHelloWorld
```
You will see straight away this:

![picture](https://github.com/ymwjbxxq/hello_serverless_rust/blob/main/warning_after_setup.png)

Open the Cargo.toml file and change the value for name into serverless_hello_world.

### Some Rust explaination ###

Considerig that this is my HelloWorld I will explain the code as I learn it.

*The Imports*
```
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;
```
You need to impor the library. Rust use [Cargo](https://doc.rust-lang.org/cargo/) and to install a library  you need to run the following command:
```
cargo add <library> 
```
and they will be added to the Cargo.toml file
```
[dependencies]
lambda_runtime = "0.4.1"
log = "0.4.14"
serde = "1.0.130"
simple_logger = "1.13.0"
tokio = "1.12.0"
```

*Import your code*

We are not writing all the code in one file and as in this case we use two typings for the request/response of the Lambda function.

Rust as the concept of [Modules](https://doc.rust-lang.org/rust-by-example/mod.html). A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.

As my first day I hit my head on this and it took me a couple of coffes to grasp it.

The structure of this Lambda function is:

![picture](https://github.com/ymwjbxxq/hello_serverless_rust/blob/main/lambda-structure.png)

As you can see I have create a file call mod.rs in the dtos/ folder. In Rust you must explicitly build the module tree. This file contains the declaration of my struct
```
pub mod request;
pub mod response;
```
The `mod` keyword declares a submodule. Now we can access it in this way:
```
mod dtos;
use dtos::request::Request;
use dtos::response::Response;
```
Request/Response are two struct. Coming from OOP a Rust struct is like an object, you do not need to create them but just instanciate.
 
*Lambda handler*

I took the syntax from AWS but there are few things interesting:
```
#[tokio::main]
```
[Tokio](https://docs.rs/tokio/1.12.0/tokio/) is an event-driven, non-blocking I/O platform for writing asynchronous applications with the Rust programming language and it is needed to mark the entry point async.
```
SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();
```
It is required to enable CloudWatch error logging by the runtime 

### Deploy ###
```
./deploy.sh
```
It will take care to build, create the zip and run for you the sam deploy.

### Cleanup ###
```
sam delete --stack-name STACK_NAME
```
