# How to work with the Book


Please, take a look at the CLI docs for more information and some neat tricks.

```bash
mdbook build
```

This is the command you will run to render your book, it reads the SUMMARY.md file to understand the structure of your book, takes the markdown files in the source directory as input and outputs static html pages that you can upload to a server.

```bash
mdbook watch
```

When you run this command, mdbook will watch your markdown files to rebuild the book on every change. This avoids having to come back to the terminal to type mdbook build over and over again.

```bash
mdbook serve
```

Does the same thing as mdbook watch but additionally serves the book at http://localhost:3000 (port is changeable) and reloads the browser when a change occurs.

```bash
mdbook clean
```

Delete directory in which generated book is located.