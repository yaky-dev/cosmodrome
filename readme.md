# Cosmodrome

Build a static website and a [Gemini](https://gemini.circumlunar.space/docs/faq.gmi) capsule in parallel

## Quick Setup

[Install Rust](https://www.rust-lang.org/learn/get-started) 

Clone the repo `git clone https://github.com/yaky-dev/cosmodrome`

Go to the cloned repo `cd cosmodrome`

Build Cosmodrome `cargo build --release`

Go back up one level `cd ..`

Create a directory for the new web site and gemini capsule `mkdir new-site-capsule`

Go to the directory `cd new-site-capsule`

Copy Cosmodrome's executable to current directory `cp ../cosmodrome/target/release/cosmodrome .` (or, you can just call Cosmodrome from this path, I find it easier to copy)

Initialize the source `./cosmodrome init`

Build the static website and the gemini capsule `./cosmodrome build`

All of the website files are in `srv/www`, and all of the gemini capsule files are in `srv/gemini`

The only thing left is to host both

## Structure
```
root
|- src
|  |- _wrapper.gmi
|  |- _wrapper.html
|  |- index.gmi
|  \- ... all other files
|- srv
|  |-gemini
|  \-www
|- gemini
\- www
```

`src` contains the site/capsule source, including pages, images, and other files.
All files that start with `_` are ignored.
Source pages have the `.gmi` extension and use [gemtext](https://gemini.circumlunar.space/docs/gemtext.gmi) markup (similar to Markdown, but simpler). Links to images are automatically converted to HTML `img` tags for Web pages.

Every source page gets wrapped in `_wrapper.gmi` (for Gemini) or `_wrapper.html` (for Web). Wrapper must have a line `<!-- CONTENT -->`, which is replaced by the page content at build time. This setup is similar to how `_Layout.cshtml` works for ASP.NET MVC pages.

`srv` contains distributable files, `gemini` for gemini capsule, `www` for website.

`www` contains extra files for the Web, such as CSS, JS or any Web-specific content. It gets copied to `dist/www` at the end of the build.
`gemini` contains extra files for Gemini. It gets copied to `dist/gemini` at the end of the build.