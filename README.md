<h1><center>ri-casper-js - Casper JS Wrapper</h1>

CasperJS is an open source application to execute Web Scraping.

This project was made to execute CasperJS in Linux OS. So, this project is just a 'wrapper class' to execute CasperJS engine.

The open source CasperJS project have a script to execute CasperJS, but this was written in Python.
We wouldn't like to depends of a Python runtime, so we rewritted the community script in Rust. This way, we can build the project getting a binary executable, without a dependency of a specific runtime.

Actually, our application just works for Linux OS, because we use a library that just works for Linux OS too.

In the Python script, the method 'execvp' is used from 'os' lib.
This method serve to replace the current proccess by a new proccess. This way, just run a new child proccess will not solve our problem. So, we are using a lib called 'exec' to get this feature, like on Python and C languages.

<h2>How can you build the project?</h2>

If you want to build the project, you can run this command:

```
cargo build --release --target x86_64-unknown-linux-musl
```
