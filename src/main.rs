/*
TODO use rouille::extension_to_mime and rexiv2's mime detection
TODO use rouille's cache control (`rouille::Response::with_no_cache` and
`rouille::Response::with_private_cache`)

TODO read POST input using `rouille::input`
TODO read URL querystring using `rouille::Request::raw_url` and Servo's
`url::Url::query_pairs`

TODO use JWT for authentication. send JWT using Set-Cookie
https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
TODO get JWT cookie using `rouille::input::cookies`

TODO run command for thumbnail generation and other image manipulation
`std::process::Command`
thumbnail generation

TODO use rexiv2 for handling image metadata. use non-EXIF formats too?
TODO list image formats in README: http://dev.exiv2.org/projects/exiv2/wiki/Supported_image_formats
*/

#[macro_use]
extern crate rouille;

mod fileio;

fn main() {
    let port = 8000;
    println!("See it on http://localhost:{}", port);

    // Start a server that accepts connections from the given port
    // It calls the closure with the parameter `request`, which contains
    // the path requested and other information about the client.
    rouille::start_server(format!("localhost:{}", port), move |request| {

    // Send the client a rouille:Response depending on the requested path 
    router!(request,

        // If the client requests the path 'GET /', send HTML containing a link.
        (GET) (/) => {
            rouille::Response::html("<a href='/pics/all'>See all pics</a>")
        },

        // Send list of links and metadata of all images on the server
        (GET) (/pics/all) => {
            let mut response: String = "imgs: ".into();
            for s in fileio::list_images().iter() {
                // Create a new line of HTML for each image
                response = response + &format!("<p><a href='/pics/all/{}'>{}</a></p>", s, s)
            }

            // TODO convert list of strings into HTML using handlebar template
            rouille::Response::html(response)
        },

        // Send title and download link for the image with the given `id` 
        (GET) (/pics/all/{id: String}) => {
            // TODO sanitize id; make sure it's good to embed as HTML
            // TODO sanitize id: make sure it is a valid filepath to an image
            let title = fileio::read_title(&id);
            let filename = id;
            rouille::Response::html(format!("title: {:}, <a href='/download/all/{:}'>Download</a>", title, filename))
        },

        // If it exists, send image thumbnail as a file response for client download.
        // Otherwise, send the io error.
        (GET) (/thumbnail/all/{id: String}) => {
            match fileio::get_thumbnail_file(id) {
                Ok(f) => rouille::Response::from_file("text/plain", f),
                Err(e) => rouille::Response::html(format!("{:?}", e))
            }
        },

        // Similarly, send the image as a file. 
        (GET) (/download/all/{id: String}) => {
            match fileio::get_download_file(id) {
                Ok(f) => rouille::Response::from_file("text/plain", f),
                Err(e) => rouille::Response::html(format!("{:?}", e))
            }
        },

        // Send a 404 response to a request for any other path. 
        _ => rouille::Response::empty_404()
    )});
}
