# Down2Me - Media downloader

## Running the project in development mode

There are two ways to run the project in development mode:

- Using a local Rust installation:
    - Install Rust locally if not installed yet. The install instructions can be found in https://www.rust-lang.org/tools/install.
    - Install a nightly version of Rust using the command `rustup default nightly`.
    - Install the cargo watch package for recompiling changed files automatically using the command `cargo install cargo-watch`.
    - Start the project with `cargo watch -x run`.

- Using the Dockerfile provided:
   - Run the command `docker build -t down2me -f dev.Dockerfile .` to build the image locally.
   - Run `docker run -d --name down2me -p 4000:4000 -v "$(pwd)"/.:/src/app down2me sh -c "cargo watch -x run"` to start the project Docker image.
   - Tip: You can follow the container logs by running `docker logs down2me -f`.