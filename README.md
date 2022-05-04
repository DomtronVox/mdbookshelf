
mdBookShelf is build around the [mdBook](https://github.com/rust-lang/mdBook) **crate**, and provides a single static website that can generate multiple mdBooks and organize them onto virtual shelves. It also allows for having PDFs on the virtual shelves.

* Crawls a source directory (default is "bookshelf") for PDFs and book.toml
* Builds relevent files into the build directory (default is "build")
    * Builds all mdbooks outputting to a folder in the build directory.
    * Copies all PDFs to the build directory.
    * **(WIP)** Builds an index.html and associated files that allows navigating to any mdbook or PDF.

# Usage
To use mdBookShelf, create a directory and add the bookshelf folder to it. Under this folder you can have folders with whatever names you would like your virtual shelves to be labled. Under those "shelf directories" you place PDF files or mdBook directories. The below shows how the directory stucture should look:

* Project root directory
   * bookshelf/
      * "Shelf 1"/
         * my mdbook/
            * book.toml
            * ...
         * random.pdf
      * "Shelf 2"/
         * ...
         
You can setup mdBook directories as normal, but mdBookShelf does overwrite some things so they are essentially ignored.

1. Each mdBook's configured build directory is overwritten with one that corisponds to mdBookShelf's build directory.
1. Each mdBook is provided a header file replacing the header file configured header in the book. This allows us to add a link back to the root bookshelf at the top of each page. If you want your own header then you should configure it in mdBookShelf's configuration file instead.

Once your mdbooks are all setup and your PDFs are in place you can run ``mdbookshelf build`` or ``mdbookshelf serve`` to either build, or build and serve the whole site as a local host server.
