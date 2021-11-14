extern crate fs_extra;

use std::env;
use std::fs;
use std::path::Path;

mod apperror;
use apperror::AppError;

const SRC_DIR: &str = "src";
const DIST_DIR: &str = "dist";
const DIST_WWW_DIR: &str = "dist/www";
const DIST_GEM_DIR: &str = "dist/gem";
const EXTRA_WWW_DIR: &str = "www";
const HTML_WRAPPER_PATH: &str = "src/_wrapper.html";
const GMI_WRAPPER_PATH: &str = "src/_wrapper.gmi";
const HTML_EXT: &str = "html";
const GMI_EXT: &str = "gmi";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && &args[1] == "init" {
        let base_dir = if args.len() >= 3 { Path::new(&args[2]) } else { Path::new("") };
        match init(base_dir) {
            Ok(()) => println!("Init completed!"),
            Err(error) => println!("Error running init: {}", error)
        };
    } else if args.len() >= 2 && &args[1] == "build" {
        let base_dir = if args.len() >= 3 { Path::new(&args[2]) } else { Path::new("") };
        match build(base_dir) {
            Ok(()) => println!("Build completed!"),
            Err(error) => println!("Error running build: {}", error)
        };
    } else if args.len() >= 2 && &args[1] == "publish" {
        // TODO
    } else if args.len() >= 3 && &args[1] == "new-post" {
        // TODO
    } else {
        println!("Cosmodrome");
        println!("Build and publish a static web site and a gemini capsule in parallel");
        println!();
        println!("Usage:");
        println!("init [path]                Initialize directories and required files for a specified path.");
        println!("build [path]               Build a website and a gemini capsule from a specified path.");
        // println!("publish                    Publish according to publish.conf");
        // println!("new-post [name]            Add a post with specified name");
        println!();
        println!("Paths are optional and default to current directory.");
        println!();
    }
}

fn init(base_dir: &Path) -> Result<(), AppError> {
    fs::create_dir(&base_dir)?;
    let src_dir = base_dir.join(SRC_DIR);
    fs::create_dir(&src_dir)?;
    // HTML wrapper page
    let html_wrapper_path = &src_dir.join(HTML_WRAPPER_PATH);
    fs::write(html_wrapper_path, "<!DOCTYPE html>
    <html>
    <head>
    <meta charset='utf-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1'>
    <link rel='stylesheet' type='text/css' href='/site.css'>
    <title>Cosmodrome site</title>
    </head>
    <body>
    <div class='header'>
    <h1><a href='/'>Cosmodrome Site</a></h1>
    <a href='/posts'>Posts</a>
    </div>
    <div class='content'>
    <!-- CONTENT -->
    </div>
    </body>
    </html>")?;
    // Gemtext wrapper page
    let gmi_wrapper_path = &src_dir.join(GMI_WRAPPER_PATH);
    fs::write(gmi_wrapper_path, "# Cosmodrome capsule
    => /posts/index.gmi Posts

    <!-- CONTENT -->
    ")?;
    // Index page
    let index_path = &src_dir.join("index.gmi");
    fs::write(index_path, "### 3\n\n## 2\n\n# 1\n\nPoehali!")?;
    // Posts page
    let posts_dir = &src_dir.join("posts");
    fs::create_dir(posts_dir)?;
    let posts_index_path = &posts_dir.join("index.gmi");
    fs::write(posts_index_path, "Posts:")?;
    // Additional WWW resources
    let www_dir = base_dir.join(EXTRA_WWW_DIR);
    fs::create_dir(&www_dir)?;
    // CSS
    let site_path = &www_dir.join("site.css");
    fs::write(site_path, "body
    {
        background-color: #444;
        color: #ccc;
    }")?;
    Ok(())
}

fn build(base_dir: &Path) -> Result<(), AppError> {
    let src_dir = &base_dir.join(SRC_DIR);
    // Clean up and re-create dist dirs
    let dist_dir = &base_dir.join(DIST_DIR);
    fs::remove_dir_all(&dist_dir).ok();
    fs::create_dir_all(&base_dir.join(DIST_WWW_DIR))?;
    fs::create_dir_all(&base_dir.join(DIST_GEM_DIR))?;
    // Recursively read contents of src and build WWW site and Gemini capsule
    build_dir(&src_dir, &base_dir)?;
    // Overlay extra WWW files such as apps, icons and stylesheets
    let extra_www_dir = base_dir.join(EXTRA_WWW_DIR);
    fs_extra::copy_items(&[&extra_www_dir], &dist_dir, &fs_extra::dir::CopyOptions::new()).expect("Failed to copy WWW to dist dir");
    Ok(())
}

fn build_dir(dir: &Path, base_dir: &Path) -> Result<(), AppError> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            build_dir(&entry_path, &base_dir)?;
        } else {
            build_file(&entry_path, &base_dir)?;
        }
    }
    Ok(())
}

fn build_file(src_file_path: &Path, base_dir: &Path) -> Result<(), AppError> {
    if src_file_path.extension().unwrap_or(std::ffi::OsStr::new("")) == GMI_EXT
    {
        // Build a HTML page
        build_html_page(&src_file_path, &base_dir).unwrap_or_else(|error| return println!("Error building HTML page: {}", error));
        // Build a Gemini page
        build_gmi_page(&src_file_path, &base_dir).unwrap_or_else(|error| println!("Error building Gemini page: {}", error));
    } else if !src_file_path.file_name().unwrap().to_str().unwrap().starts_with("_") {
        copy_file(&src_file_path, &base_dir).unwrap_or_else(|error| println!("Error copying file: {}", error));
    }
    Ok(())
}

fn build_html_page(src_file_path: &Path, base_dir: &Path) -> Result<(), AppError> {
    println!("Building HTML page from {}...", src_file_path.to_str().unwrap_or(""));
    // Determine the path of the HTML page
    let rel_file_path = src_file_path.strip_prefix(base_dir).unwrap().strip_prefix(SRC_DIR).unwrap();
    let mut html_file_path = base_dir.join(DIST_WWW_DIR).join(rel_file_path);
    html_file_path.set_extension(HTML_EXT);
    fs::create_dir_all(html_file_path.parent().unwrap())?;
    // Convert gemtext into HTML
    let gmi_content = fs::read_to_string(src_file_path)?;
    let gmi_lines = gmi_content.lines();
    let mut html_lines = Vec::<String>::new();
    let mut ul = false; // <ul> tag is open
    let mut pre = false; // <pre> tag is open
    for gmi_line in gmi_lines {
        let gmi_line = gmi_line.trim();
        if gmi_line.starts_with("```") {
            if pre {
                html_lines.push(format!("</pre>"));
                html_lines.push(format!("</div>"));
            } else {
                html_lines.push(format!("<div>"));
                html_lines.push(format!("<pre>"));
            }
            pre = !pre;
            continue;
        }
        if pre {
            html_lines.push(gmi_line.to_string());
            continue;
        }
        // Split into prefix and contents
        let (prefix, content) = match gmi_line.split_once(' ') {
            Some((prefix, content)) => (prefix, content),
            None => ("", gmi_line)
        };
        if prefix == "*" {        
            // Open <ul> tag when starting a list
            if !ul {
                html_lines.push(format!("<div>"));
                html_lines.push(format!("<ul>"));
                ul = true;
            }
            html_lines.push(format!("<li>{}</li>", content));
            continue;        
        }
        // Close <ul> tag when ending a list
        if ul {
            html_lines.push(format!("</ul>"));
            html_lines.push(format!("</div>"));
            ul = false;
        }
        // Content line
        html_lines.push(format!("<div>"));
        match prefix {
            // Headers
            "#" => {
                html_lines.push(format!("<h1>{}</h1>", content));
            },
            "##" => {
                html_lines.push(format!("<h2>{}</h2>", content));
            },
            "###" => {
                html_lines.push(format!("<h3>{}</h3>", content));
            },
            // Quote
            ">" => {
                html_lines.push(format!("<q>{}</q>", content));
            },
            // Link or image
            "=>" => {
                let (link,desc) = match content.split_once(' ') {
                    Some((link, desc)) => (link, desc),
                    None => (content, "")
                };
                let link = link.to_lowercase();
                if link.ends_with(".jpg")
                || link.ends_with(".gif")
                || link.ends_with(".png")
                || link.ends_with(".svg")
                || link.ends_with(".webp") {
                    html_lines.push(format!("<img src=\"{}\">{}</img>", link, desc));
                } if link.ends_with(".gmi") {
                    html_lines.push(format!("<a href=\"{}\">{}</a>", link.replace(".gmi", ".html"), desc));
                } else {
                    html_lines.push(format!("<a href=\"{}\">{}</a>", link, desc));
                }
            },
            // Text or a line break
            _ => {
                if gmi_line.is_empty() {
                    html_lines.push(format!(""));
                } else {
                    html_lines.push(format!("<p>{}</p>", gmi_line));
                }
            }
        }
        html_lines.push(format!("</div>"));
    }
    let html_content = html_lines.join("\n");
    // Wrap HTML page with header and footer
    let html_wrapper_path = &base_dir.join(HTML_WRAPPER_PATH);
    let html_wrapper = fs::read_to_string(html_wrapper_path)?;
    let html_wrapper_split = html_wrapper.splitn(2, "<!-- CONTENT -->").collect::<Vec<&str>>();
    if html_wrapper_split.len() < 2 {
        return Err(AppError::new("HTML wrapper was split incorrectly"));
    }
    let html_header = html_wrapper_split[0];
    let html_footer = html_wrapper_split[1];
    let html_page = format!("{}{}{}", html_header, html_content, html_footer);
    // Save the HTML page
    fs::write(html_file_path, html_page)?;
    // Done
    println!("Done");
    Ok(())
}

fn build_gmi_page(src_file_path: &Path, base_dir: &Path) -> Result<(), AppError> {
    println!("Building gemtext page from {}...", src_file_path.to_str().unwrap_or(""));
    let rel_file_path = src_file_path.strip_prefix(base_dir).unwrap().strip_prefix(SRC_DIR).unwrap();
    let gmi_file_path = base_dir.join(DIST_GEM_DIR).join(rel_file_path);
    fs::create_dir_all(gmi_file_path.parent().unwrap())?;
    // Wrap gemtext page with header and footer
    let gmi_content = fs::read_to_string(src_file_path)?;
    let gmi_wrapper_path = &base_dir.join(GMI_WRAPPER_PATH);
    let gmi_wrapper = fs::read_to_string(gmi_wrapper_path)?;
    let gmi_wrapper_split = gmi_wrapper.splitn(2, "<!-- CONTENT -->").collect::<Vec<&str>>();
    if gmi_wrapper_split.len() < 2 {
        return Err(AppError::new("Gemtext wrapper was split incorrectly"));
    }
    let gmi_header = gmi_wrapper_split[0];
    let gmi_footer = gmi_wrapper_split[1];
    let gmi_page = format!("{}{}{}", gmi_header, gmi_content, gmi_footer);
    // Copy gemtext file 
    fs::write(gmi_file_path, gmi_page)?;
    Ok(())
}

fn copy_file(src_file_path: &Path, base_dir: &Path) -> Result<(), AppError> {
    let rel_file_path = src_file_path.strip_prefix(base_dir).unwrap().strip_prefix(SRC_DIR).unwrap();
    let www_file_path = base_dir.join(DIST_WWW_DIR).join(rel_file_path);
    let gem_file_path = base_dir.join(DIST_GEM_DIR).join(rel_file_path);
    fs::create_dir_all(www_file_path.parent().unwrap())?;
    println!("Copying file: {}", &src_file_path.to_str().unwrap_or(""));
    fs::copy(&src_file_path, &www_file_path)?;
    fs::copy(&src_file_path, &gem_file_path)?;
    Ok(())
}