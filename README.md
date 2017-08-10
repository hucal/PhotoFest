# PhotoFest

PhotoFest is a secure self-hosted flat-file photo collection webapp.
I am writing it as an exercise in secure coding and HTML5 visual design.

## Features

PhotoFest has: 

- a decent authentication system, including multiple users with various permissions
- good HTML5 layout without Javascript
- an easy to use and flexible API
- support for many image formats, even obscure ones
- YAML based storage (there is no database)
- TODO

## Screenshots

TODO

## Setup and Administration

TODO:

- install dependencies
- compile server
- server secret
- root user setup
- run server
- make sure filesystem has the right permissions
- view logs
- stop server
- backup configuration and files

## API

- Server may send HTML or YAML.
- For all requests, except `/login`, the client must send a JWT in as a POST field.
- The server may respond with a permission error if the JWT is invalid or if the user is attempting to access forbidden data.
- In case of the following "offenses", the server logs the request metadata and responds with random noise and an appropriate status code.
    - Page range out of range
    - Unknown querystring or metadata field
    - Unknown username
    - Unparseable request
    - Overly large request

1. `GET /login`: (HTML only)
    - server sends login form
1. `POST /login`:
    - client sends username and password hash
    - server sends "invalid username or password" or valid JWT
1. `POST /newuser`:
    - client sends username and password hash and new user permissions
    - server adds new user if the JWT user has permission to create users with the given permissions.
1. `POST /changepassword`:
    - client sends username and new password hash
    - server changes password if the JWT user has permission to change the given user's password.
1. `POST /removeuser`:
    - client sends username
    - server removes user if the JWT user has permission to remove that type of user
1. `POST /logout`:
    - server invalidates JWT HMAC
1. `GET /albums?p=Pp=N&n=N&f=F&o=asc\des`: 
    - server uses default values for absent fields
    - server responds with page `P` containing at most `N` elements sorted by metadata `F` of all albums readable by the user.
    - response includes total number of albums, and name and thumbnail URLs for each album 
1. `GET /pics/X?p=Pp=N&n=N&f=F&o=asc\des`: 
    - as above. server responds with album contents for album `X`. 
    - response only includes one thumbnail for each picture.
1. `GET /albums?p=Pp=N&n=N&f=F&o=asc\des&q=Q&m=M&c=true\false`: 
    - server uses default values for absent fields
    - as above. server responds with album search results for query `Q` in field `M`, search may be case insensitive or not.
1. `GET /pics?p=Pp=N&n=N&f=F&o=asc\des&q=Q&m=M&c=true\false&a=A`: 
    - as above. server responds with search results for pictures in in albums `A` or in the special album `all`.
1. `GET /pics/X/Y`: 
    - server responds with large thumbnail URL and metadata for picture named `Y` in album `X`: 
1. `GET /thumbnail/X/Y?S=true`: 
    - server responds with thumbnail for image `Y` in album `X` of size `S`: 
1. `GET /modifypics?f=F&x=X&p=P`: 
    - server modifies pictures `P` using action `F` and optional data `X`: 
    - action `F` includes rotate `X` degrees, remove, move pictures to album `X`, tag with tag `X`, remove tag `X`, rename to `X`: 
    - server can only rename one picture at a time
1. `GET /modifypics?f=F&x=X&a=A`: 
    - as above. server modifies all pictures in albums `A`.
1. `GET /download/X/Y`: 
    - server responds with picture named `Y` in album `X`: 
1. `GET /download/X?l=L`: 
    - server responds with album `X` as a zip file with the given level `L` of compression
    - client should see a progress bar
1. `POST /upload`: 
    - client sends an album name and either sends a link or uploads files
        - files may a group of images or zip archives
        - link may be within the server's import directory or an internet address (not on the local network)
    - client should see a progress bar
    - client cannot upload to special albums
    - server responds with list of duplicate images that were not added to album
1. `GET /userlist`:
    - server sends list of usernames administered by the current user and their permissions
1. `GET /settings`: (HTML only)
    - server sends forms for modifying user settings and, if applicable, other users
    - server shows log and status to user `root`
    - client may change their prefered HTML theme, YAML formatting options, and album/picture sorting order

### Albums

If an album does not exist, it will be created.
Empty albums are allowed. 

Special albums include:

- `all` stores every picture
- `recent` stores recently added pictures
- `unsorted` is the default upload album

Metadata includes:

- name
- number of photos 
- total file size of pictures
- total number of pixels

### Pictures

Pictures may be in any format supported by the (magical image manipulation library).
(Do they all support EXIF tags?) 
If a picture format does not support EXIF tags, any tag operations on it will be reported as warnings. 
Some formats include:

Metadata includes:

- name
- file size
- format
- dimension (total number of pixels, width, or height)
- date added to PhotoFest
- EXIF date taken
- EXIF tag (a certain tag or has/hasn't tag)
- other EXIF tags (ISO level, GPS, etc...)
- perceptual hash (can it be sorted?)

If a picture lacks the particular EXIF tag or if it does not support EXIF tags,
it will be sorted last. 

### User Permissions

Every album has an owner.
Every user has a permission level that determines what they can read and write. 
A user of level `N` can:

- create user of level `N-1` or lower
- change password of self or any user of level `N-1` or lower
- remove any user of level `N-1` or lower
- view the media belonging to self or users of level `N-1` or lower
- change the ownership of an album to self or any users of level `N-1` or lower

Usernames `root` and `guest` are reserved.
User of level `0` cannot create or modify users of level `-1`.
These are the names for each level of user:

| Level | Username | Notes |
|-------|------|-------|
| -1 | guest | Only one created during setup. No write access. No password. |
| 0  | user  |
| 1  | admin |
| 2  | root  | Only one created during setup. |

### JWT Contents

The server ignores the `alg` field in the JWT header.
Every JWT token must contain the following information:

- username
- dates (TODO)

## Internals

PhotoFest is written in Rust using YAML files for persistance. 

TODO: library and architecture overview

Rouille's performance: https://github.com/tomaka/rouille/issues/130
- might be optimized for long-running requests (perfect!)
- TODO: test upload and download speed of Rust's other web frameworks
