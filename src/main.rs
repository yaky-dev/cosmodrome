// Use fs_extra crate for recursive file copy function
extern crate fs_extra; 

use std::env;
use std::fs;
use std::path::Path;

// Custom generic error for application flow
mod apperror;
use apperror::AppError;

// Pre-defined directory structure
const SRC_DIR: &str = "src";
const SRV_DIR: &str = "srv";
const SRV_WWW_DIR: &str = "srv/www";
const SRV_GEM_DIR: &str = "srv/gemini";
const EXTRA_WWW_DIR: &str = "www";
const EXTRA_GEM_DIR: &str = "gemini";
const HTML_WRAPPER_PATH: &str = "src/_wrapper.html";
const GMI_WRAPPER_PATH: &str = "src/_wrapper.gmi";
const HTML_EXT: &str = "html";
const GMI_EXT: &str = "gmi";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && &args[1] == "init" {
        let base_dir = if args.len() >= 3 { Path::new(&args[2]) } else { Path::new("") };
        println!("Initializing directory {}", base_dir.to_str().unwrap());
        match init(&base_dir) {
            Ok(()) => println!("Initialization completed!"),
            Err(error) => eprintln!("Initialization error: {}", error)
        };
    } else if args.len() >= 2 && &args[1] == "build" {
        let base_dir = env::current_dir().unwrap().join(if args.len() >= 3 { Path::new(&args[2]) } else { Path::new("") });
        match build(&base_dir) {
            Ok(()) => println!("Build completed!"),
            Err(error) => eprintln!("Build error: {}", error)
        };
    } else {
        println!("Cosmodrome");
        println!("Build a static web site and a gemini capsule in parallel");
        println!();
        println!("Usage:");
        println!("init [path]                Initialize directories and required files for a specified path.");
        println!("build [path]               Build a website and a gemini capsule from a specified path.");
        println!();
        println!("Paths are optional and default to current directory.");
    }
    println!();
}

/// Initialize necessary source files in specified directory
fn init(base_dir: &Path) -> Result<(), AppError> {
    if base_dir.is_dir() {
        if base_dir.exists() {
            println!("Base directory ({}) already exists", base_dir.display());
        } else {
            print!("Initializing base directory ({})... ", base_dir.display());
            fs::create_dir(&base_dir)?;
            println!("Done!");
        }
    }
    let src_dir = base_dir.join(SRC_DIR);
    if src_dir.exists() {
        println!("Source directory ({}) already exists", src_dir.display());
    } else {
        print!("Initializing source directory ({})... ", src_dir.display());
        fs::create_dir(&src_dir)?;
        println!("Done!");
    }
    // HTML wrapper page
    let html_wrapper_path = &base_dir.join(HTML_WRAPPER_PATH);
    if html_wrapper_path.exists() {
        println!("HTML wrapper ({}) already exists", html_wrapper_path.display())
    } else {
        print!("Initializing HTML wrapper ({})... ", html_wrapper_path.display());
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
<a href='/about.html'>About</a>
</div>
<div class='content'>
<!-- CONTENT -->
</div>
<div class='footer'>
Built with <a href='https://github.com/yaky-dev/cosmodrome'>Cosmodrome</a>
</div>
</body>
</html>")?;
        println!("Done!");
    }
    // Gemtext wrapper page
    let gmi_wrapper_path = &base_dir.join(GMI_WRAPPER_PATH);
    if gmi_wrapper_path.exists() {
        println!("Gemtext wrapper ({}) already exists", gmi_wrapper_path.display());
    } else {
        print!("Initializing Gemtext wrapper ({})... ", gmi_wrapper_path.display());
        fs::write(gmi_wrapper_path, "# Cosmodrome capsule
=> / Home
=> /about.gmi About

<!-- CONTENT -->

Build with Cosmodrome
=> https://github.com/yaky-dev/cosmodrome Cosmodrome on GitHub")?;
        println!("Done!");
    }
    // Index page
    let index_path = &src_dir.join("index.gmi");
    if index_path.exists() {
        println!("Index page ({}) already exists", index_path.display());
    } else {
        print!("Initializing index page ({})... ", index_path.display());
        fs::write(index_path, "3
### 2
## 1
# Poehali!")?;
        println!("Done!");
    }
    // About page
    let about_path = &src_dir.join("about.gmi");
    if about_path.exists() {
        println!("About page ({}) already exists", about_path.display());
    } else {
        print!("Initializing about page ({})... ", about_path.display());
        fs::write(about_path, "## About

This is a sample page displaying gemtext syntax

### Posts:
* Post 1
* Post 2
* Post 3

A quote
> The Earth is blue... how wonderful. It is amazing
- Yuri Gagarin

```
Some preformatted table or ascii art, perhaps?
```")?;
        println!("Done!");
    }
    // Additional WWW resources
    let www_dir = base_dir.join(EXTRA_WWW_DIR);
    if www_dir.exists() {
        println!("WWW extras directory ({}) already exists", www_dir.display());
    } else {
        print!("Initializing WWW extras directory ({})... ", www_dir.display());
        fs::create_dir_all(&www_dir)?;
        println!("Done!");
    }
    // Additional Gemini resources
    let gem_dir = base_dir.join(EXTRA_GEM_DIR);
    if gem_dir.exists() {
        println!("Gemini extras directory ({}) already exists", gem_dir.display());
    } else {
        print!("Initializing Gemini extras directory ({})... ", gem_dir.display());
        fs::create_dir_all(&gem_dir)?;
        println!("Done!");
    }
    // CSS
    let site_css_path = &www_dir.join("site.css");
    if site_css_path.exists() {
        println!("Site CSS ({}) already exists", site_css_path.display());
    } else {
        print!("Initializing site CSS ({})... ", site_css_path.display());
        fs::write(site_css_path, "body
        {
            background-color: #ffc;
            color: #042161;
        }")?;
        println!("Done!");
    }
    Ok(())
}

/// Build a static website and a Gemini capsule from specified directory
fn build(base_dir: &Path) -> Result<(), AppError> {
    let src_dir = &base_dir.join(SRC_DIR);
    if !src_dir.exists() {
        return Err(AppError::new(format!("Source directory ({}) not found. Nothing to build", src_dir.display()).as_str()));
    }
    // Clean up and re-create srv dirs
    print!("Cleaning up... ");
    let srv_dir = &base_dir.join(SRV_DIR);
    if srv_dir.exists() {
        fs::remove_dir_all(&srv_dir)?;
    }
    fs::create_dir_all(&base_dir.join(SRV_WWW_DIR))?;
    fs::create_dir_all(&base_dir.join(SRV_GEM_DIR))?;
    println!("Done!");
    // Recursively read contents of src and build WWW site and Gemini capsule
    build_dir(&src_dir, &base_dir)?;
    // Overlay extra WWW files such as apps, icons and stylesheets
    print!("Copying extra WWW files... ");
    let extra_www_dir = base_dir.join(EXTRA_WWW_DIR);
    fs_extra::copy_items(&[&extra_www_dir], &srv_dir, &fs_extra::dir::CopyOptions::new()).expect("Failed to copy WWW to srv dir");
    // Overlay extra Gemini files such as apps, icons and stylesheets
    print!("Copying extra Gemini files... ");
    let extra_gem_dir = base_dir.join(EXTRA_GEM_DIR);
    fs_extra::copy_items(&[&extra_gem_dir], &srv_dir, &fs_extra::dir::CopyOptions::new()).expect("Failed to copy Gemini to srv dir");
    println!("Done!");
    Ok(())
}

/// Recursively process all items in the directory
fn build_dir(dir: &Path, base_dir: &Path) -> Result<(), AppError> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        // Do not build things like page wrappers
        let entry_is_ignored = entry_path.file_name().unwrap().to_str().unwrap().starts_with("_");
        if !entry_is_ignored
        {
            if entry_path.is_dir() {
                build_dir(&entry_path, &base_dir)?;
            } else {
                build_file(&entry_path, &base_dir)?;
            }
        }
    }
    Ok(())
}

/// Decide what to do with a file: create an HTML page, create a Gemini page or just copy it
fn build_file(src_file_path: &Path, base_dir: &Path) -> Result<(), AppError> {
    if src_file_path.extension().unwrap_or(std::ffi::OsStr::new("")) == GMI_EXT
    {
        // Build a HTML page
        build_html_page(&src_file_path, &base_dir).unwrap_or_else(|error| eprintln!("HTML page build error: {}", error));
        // Build a Gemini page
        build_gmi_page(&src_file_path, &base_dir).unwrap_or_else(|error| eprintln!("Gemini page build error: {}", error));
    } else {
        copy_file(&src_file_path, &base_dir).unwrap_or_else(|error| println!("Copy error: {}", error));
    }
    Ok(())
}

/// Use specified source page to create an HTML page in srv/www
fn build_html_page(src_file_path: &Path, base_dir: &Path) -> Result<(), AppError> {
    print!("Building HTML page from {}... ", src_file_path.to_str().unwrap_or(""));
    // Determine the path of the HTML page
    let rel_file_path = src_file_path.strip_prefix(base_dir).unwrap().strip_prefix(SRC_DIR).unwrap();
    let mut html_file_path = base_dir.join(SRV_WWW_DIR).join(rel_file_path);
    html_file_path.set_extension(HTML_EXT);
    fs::create_dir_all(html_file_path.parent().unwrap())?;
    // Convert gemtext into HTML
    let gmi_content = fs::read_to_string(src_file_path)?;
    let gmi_lines = gmi_content.lines();
    let mut html_lines = Vec::<String>::new();
    let mut ul = false; // <ul> tag is open
    let mut pre = false; // <pre> tag is open
    let mut bq = false; // <blockquote> tag is open
    for gmi_line in gmi_lines {
        let gmi_line = gmi_line.trim();
        if gmi_line.starts_with("```") {
            if pre {
                html_lines.push(format!("</pre>"));
            } else {
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
                html_lines.push(format!("<ul>"));
                ul = true;
            }
            html_lines.push(format!("<li>{}</li>", content));
            continue;        
        }
        // Close <ul> tag when ending a list
        if ul {
            html_lines.push(format!("</ul>"));
            ul = false;
        }
        if prefix == ">" {        
            // Open <blockquote> tag when starting a list
            if !bq {
                html_lines.push(format!("<blockquote>"));
                bq = true;
            }
            html_lines.push(format!("{}<br/>", content));
            continue;        
        }
        // Close <blockquote> tag when ending a blockquote
        if bq {
            html_lines.push(format!("</blockquote>"));
            bq = false;
        }        
        // Content line
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
                    html_lines.push(format!("<div><img src=\"{}\">{}</img></div>", link, desc));
                } else if link.starts_with("/") && link.ends_with(".gmi") {
                    html_lines.push(format!("<div><a href=\"{}\">{}</a></div>", link.replace(".gmi", ".html"), desc));
                } else {
                    html_lines.push(format!("<div><a href=\"{}\">{}</a></div>", link, desc));
                }
            },
            // Text or a line break
            _ => {
                if gmi_line.is_empty() {
                    html_lines.push(format!("<br/>"));
                } else {
                    html_lines.push(format!("<p>{}</p>", gmi_line));
                }
            }
        }
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
    println!("Done");
    Ok(())
}

/// Use specified source page to create a Gemini page in srv/gem
fn build_gmi_page(src_file_path: &Path, base_dir: &Path) -> Result<(), AppError> {
    print!("Building Gemini page from {}... ", src_file_path.to_str().unwrap_or(""));
    let rel_file_path = src_file_path.strip_prefix(base_dir).unwrap().strip_prefix(SRC_DIR).unwrap();
    let gmi_file_path = base_dir.join(SRV_GEM_DIR).join(rel_file_path);
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
    println!("Done!");
    Ok(())
}

// Copy specified file to both srv/www and srv/gem
fn copy_file(src_file_path: &Path, base_dir: &Path) -> Result<(), AppError> {
    let rel_file_path = src_file_path.strip_prefix(base_dir).unwrap().strip_prefix(SRC_DIR).unwrap();
    let www_file_path = &base_dir.join(SRV_WWW_DIR).join(rel_file_path);
    let gem_file_path = &base_dir.join(SRV_GEM_DIR).join(rel_file_path);
    fs::create_dir_all(www_file_path.parent().unwrap())?;
    fs::create_dir_all(gem_file_path.parent().unwrap())?;
    print!("Copying file {} ... ", &src_file_path.to_str().unwrap_or(""));
    fs::copy(&src_file_path, &www_file_path)?;
    fs::copy(&src_file_path, &gem_file_path)?;
    println!("Done!");
    Ok(())
}