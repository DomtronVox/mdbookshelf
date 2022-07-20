
mdBookShelf is built around the [mdBook](https://github.com/rust-lang/mdBook) **crate**, and provides a single static website that can generate multiple mdBooks and organize them onto virtual shelves. It also allows for having PDFs on the virtual shelves.

* Crawls a source directory (default is "bookshelf") for PDFs and book.toml
* Builds relevant files into the build directory (default is "build")
    * Builds all mdbooks outputting to a folder in the build directory.
    * Copies all PDFs to the build directory.
    * Builds an index.html and associated files that allows navigating to any mdbook or PDF.

# Usage
To use mdBookShelf, create a directory and add the bookshelf folder to it. Under this folder you can have folders with whatever names you would like your virtual shelves to be labled. Under those "shelf directories" you place PDF files or mdBook directories. The below shows how the directory stucture should look:

* Project root directory
   * bookshelf/
      * "Shelf 1"/
         * my mdbook/
            * book.toml
            * ...
         * random.pdf
         * /"Sub-shelf 1"/
            * Another random.pdf
      * "Shelf 2"/
         * ...
         
You can setup mdBook directories as normal, but mdBookShelf does overwrite some settings in the config so they are essentially ignored if you set them.

1. Each mdBook's configured build directory is overwritten with one that corresponds to mdBookShelf's build directory.
1. **Not implemented** Each mdBook is provided a header file replacing the header file configured header in the book. This allows us to add a link back to the root bookshelf at the top of each page. If you want your own header then you should configure it in mdBookShelf's configuration file instead.

Once your mdbooks are all setup and your PDFs are in place you can run ``mdbookshelf build`` or ``mdbookshelf serve`` to respectively just build, or build and then serve the whole site as a local host server.

# TODO

* Make things configurable by toml file.
    * source directory, build directory, bookshelf directory where everything under the build directory is placed.
    * Site title possibly url as well if that is relevant.
* Set MDBook header with a link that takes you back to the library index page.
* Probably should embed the PDF into a page so we can add things like a link back to the index page.
* Make template so it can be overridden.
    * At the very least we need to be able to override the styling CSS file.
    * Also make the header able to be overriden, once we implement that.
* Need to add a 404 page at least for the local host server.
* Possibly add search to the index page.
* Possibly add some kind of PDF thumbnail generator to capture the first page so they can have a cover shown on the shelf. Only issue is it wouldn't work for MDBooks as they don't really have any kind of cover.
* Look into supporting other file formats like epub.
* Look into allowing MDBook plugins to work.

