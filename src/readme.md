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

Initialize the source `cosmodrome init`

Build the static website and the gemini capsule `cosmodrome build`

All of the website files are in `dist/www`, and all of the gemini capsule files are in `dist/gem`

The only thing left is to host both

## Structure
```
root
|- dist
|  |-gem
|  \-www
|- src
|  |- _wrapper.gmi
|  |- _wrapper.html
|  |- index.gmi
|  \- ... all other files
\- www
```

`dist` contains distributable files, `gem` for gemini capsule, `www` for website.

`src` contains the site/capsule source, including pages, images, and other files.
All files that start with `_` are ignored.
Source pages have the `.gmi` extension and use [gemtext](https://gemini.circumlunar.space/docs/gemtext.gmi) markup (similar to Markdown, but simpler).
Every source page gets wrapped in `_wrapper.gmi` (for Gemini) or `_wrapper.html` (for Web). This is similar to how `_Layout.cshtml` works for ASP.NET MVC pages.
Links to images are automatically converted to HTML `img` tags for Web pages.

`www` contains extra files for web, such as CSS, JS or any web-specific content. It gets copied to `dist/www` at the end of the build.