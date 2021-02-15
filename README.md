# Down2Me - Media downloader

## About the project

This project aims to be a simple and effective media downloader. For now, it supports the major social network video download requests, such as Youtube, Twitter, Instagram, etc. The core process of the API transfer the requests received to youtube-dl package installed in the container, and this package responds with an object that contains the information about the video received in the request. All information about youtube-dl can be found in https://github.com/ytdl-org/youtube-dl.

## Available endpoints
### Index page
#### Request

`GET /`

    curl -i -H 'Accept: application/json' http://localhost:4000/

#### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 2

    [{message: "Welcome to Down2me Media Downloader API"}]

### Get video download links

#### Request

`POST /video/download`

    curl -i -H 'Accept: application/json' POST http://0.0.0.0:4000/video/download --data '{"video_url":"http://video-link.com"}'

#### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 2

    [{
        title: 'An video example',
        thumbnail: 'http://thumbnail-link.example.com',
        formats: [
            {
                ext: 'mp4',
                filesize: 212414,
                format: '1080x1920',
                height: 1920,
                width: 1080,
                url: 'http://video-download-link.example.mp4'
            }
        ],
    }]

## Running the project

There are two ways to run the project:

- Using a local Rust installation:
    - Install Rust locally if not installed yet. The install instructions can be found in https://www.rust-lang.org/tools/install.
    - Install a nightly version of Rust using the command `rustup default nightly`.
    - Start the project with `cargo run`.

- Using the Dockerfile provided:
   - Run the command `docker build -t down2me -f dev.Dockerfile .` to build the image locally.
   - Run `docker run -d --name down2me -p 4000:4000 -v "$(pwd)"/.:/src/app down2me` to start the project Docker image.

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