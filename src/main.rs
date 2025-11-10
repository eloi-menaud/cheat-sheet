use anyhow::{bail, Context, Result};
use clap::Parser;
use comrak::html::ChildRendering;
use comrak::nodes::NodeValue;
use comrak::{create_formatter, format_html, parse_document, Arena, Options};
use serde_json::{json, Value};
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use std::collections::HashSet;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};
use fs_extra::dir::{copy, CopyOptions};

const ROOT_SRC_DIR: &str = "pages";
const ROOT_DIST_DIR: &str = "site";
const PAGE_HTML_TEMPLATE: &str = "src/site/pages/page.tpl.html";

const SITE_RSC_SRC_DIR: &str = "src/site/";
const SITE_RSC_DIST_DIR: &str = ".";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    check: bool,
}

struct Page {
    tech: String,
    title: String,
    dir: PathBuf,
    index: PathBuf,
    details: Option<PathBuf>,
    id: Option<u16>,
}

impl Page {
    /// Creates a Page struct by reading the directory, validating required files, and extracting the ID.
    fn from_path(page_dir: &PathBuf) -> Result<Page> {
        let entries = fs::read_dir(page_dir)
            .with_context(|| format!("Failed to read page directory: {}", page_dir.display()))?;

        let mut index_md_found = false;
        let mut id: Option<u16> = None;

        for entry in entries {
            let path = entry.context("Failed to scan directory entry")?.path();

            if path.is_dir() {
                bail!("Page directory '{}' must only contain files, found subdirectory: {}", page_dir.display(), path.display())
            }

            let stem = path.file_stem()
                .context("Could not get file stem for path")?
                .to_str()
                .context("File stem is not valid UTF-8")?;
            let ext = path.extension()
                .context("Could not get file extension for path")?
                .to_str()
                .context("File extension is not valid UTF-8")?;

            match (stem, ext) {
                ("index", "md") => {
                    index_md_found = true;
                }
                ("details", "md") => {}
                (i, "id") => {
                    let parsed_id = i
                        .parse::<u16>()
                        .with_context(|| format!("Failed to parse ID value '{}' to u16 in file: {}", i, path.display()))?;
                    
                    if id.is_some() {
                        bail!("Duplicate ID file found in directory: {}", page_dir.display());
                    }
                    id = Some(parsed_id);
                }
                // Catch-all for unknown files
                _ => {
                    bail!(
                        "Page directory '{}' contains an unknown file: {}. Allowed files are 'index.md', 'details.md', and '{{id}}.id'.",
                        page_dir.display(),
                        path.file_name().unwrap_or_default().to_string_lossy()
                    );
                }
            }
        }

        if !index_md_found {
            bail!("Page directory '{}' is missing the required 'index.md' file", page_dir.display())
        }
        
        // Construct index.md and details.md paths based on the structure validation above
        let index_path = page_dir.join("index.md");
        let details_path = page_dir.join("details.md");
        
        let title_name = get_dir_name(&page_dir)?.to_string();
        let project_dir = page_dir.parent().context("Page directory has no parent directory")?;
        let tech_name = get_dir_name(project_dir)?.to_string();
        
        Ok(Page {
            tech: tech_name,
            title: title_name,
            dir: page_dir.clone(),
            index: index_path,
            details: if details_path.exists() {
                Some(details_path)
            } else {
                None
            },
            id,
        })
    }
    
    fn to_json(&self) -> Result<Value> {
        Ok(json!({
            "id": self.id,
            "tech": self.tech,
            "title": self.title
        }))
    }
}


fn get_dir_name<'a>(path: &'a Path) -> Result<String> {
    Ok(String::from(
        path.file_name()
            .context("Path is invalid or ends in '..'")?
            .to_str()
            .context("Path component is not valid UTF-8")?
    ))
}


/// Reads a directory, ensuring all entries are directories, and returns their paths.
fn get_direct_subdirs(dir_path: &Path) -> Result<Vec<PathBuf>> {
    let mut subdirs: Vec<PathBuf> = Vec::new();

    let entries = fs::read_dir(dir_path)
        .with_context(|| format!("Failed to read source directory: {}", dir_path.display()))?;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        let file_type = entry.file_type()
            .with_context(|| format!("Failed to get file type for: {}", path.display()))?;

        if file_type.is_dir() {
            subdirs.push(path);
        } else {
            bail!(
                "Path '{}' must only contain directories, found file: {}",
                dir_path.display(),
                path.display()
            )
        }
    }

    Ok(subdirs)
}


fn highlight_with_simple_classes(code: &str, lang_extension: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();

    let syntax = ss.find_syntax_by_extension(lang_extension).unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut cpp_html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &ss, ClassStyle::SpacedPrefixed { prefix: "hl-" });
    for line in LinesWithEndings::from(code) {
        cpp_html_generator
            .parse_html_for_line_which_includes_newline(line)
            .unwrap();
    }
    cpp_html_generator.finalize()
}



fn md_to_html(path: &PathBuf) -> Result<String> {
    
    
    create_formatter!(CustomFormatter, {
        NodeValue::CodeBlock(ref cb) => |context, node, entering| {
            if entering {
                let language = cb.info.split_whitespace().next().unwrap_or("text");

                let code = &cb.literal;
                let html_colored = highlight_with_simple_classes(code,language);
                
                context.write_str(&format!(
                    r#"<div class="code-block">
            <div class="code-header">
                <p class="language">{language}</p>
                <div><img class="copy" src="../../rsc/assets/copy.svg"></div>
            </div>
            <pre><code class="language-{language}">{html_colored}</code></pre>
            "#
                ))?;
            }
            else{
                context.write_str("</div>")?;
            }
        }
    });
    
    println!("Source: {}", path.display());
    
    
    let md = fs::read_to_string(path)
        .with_context(|| format!("Failed to read markdown file: {}", path.display()))?;

    let arena = Arena::new();
    let mut options = Options::default();
    options.render.r#unsafe = true;
    
    let root = parse_document(&arena, &md, &options);
    
    let mut html = String::new();
    let _ = CustomFormatter::format_document(root, &options, &mut html);

    Ok(html)
}

fn create_html_page(page: &Page) -> Result<()> {
    // Convert markdown to HTML
    let index_html = md_to_html(&page.index)
        .with_context(|| format!("Failed to parse 'index.md' to HTML: {}", page.index.display()))?;

    let details_html = if let Some(details) = &page.details {
        Some(md_to_html(&details)
            .with_context(|| format!("Failed to parse 'details.md' to HTML: {}", details.display()))?)
    } else {
        None
    };

    let content = if let Some(details) = details_html {
        format!("<section id='snippet'>\n{}\n</section>\n<section id='details' class='hide'>\n{}\n</section>", index_html, details)
    } else {
        index_html
    };

    // Load template
    let template = fs::read_to_string(PAGE_HTML_TEMPLATE)
        .with_context(|| format!("Failed to read HTML template: {}", PAGE_HTML_TEMPLATE))?;


    // Get ID (must be present at this stage)
    let page_id = page.id.context("Page ID is missing. It should have been generated/assigned.")?;
    let id_str = page_id.to_string();

    // Replace placeholders
    let html = template
        .replace("%TITLE%", &page.title)
        .replace("%TECH%", &page.tech)
        .replace("%ID%", &id_str)
        .replace("%CONTENT%", &content);

    // Define output path
    let dist_file_path = PathBuf::from(ROOT_DIST_DIR).join(format!("pages/{}/",page_id.to_string())).join("index.html");


    // Write
    fs::create_dir_all(dist_file_path.parent().unwrap())
        .with_context(|| format!("Failed to create dist page directory: {}", dist_file_path.parent().unwrap().display()))?;

    fs::write(&dist_file_path, html)
        .with_context(|| format!("Failed to write HTML file to: {}", dist_file_path.display()))?;

    Ok(())
}

/// Finds the first available ID and creates the ID file for the given page.
fn find_and_assign_id(page: &mut Page, existing_ids: &mut HashSet<u16>) -> Result<()> {
    // Find the first available ID between 1 and u16::MAX
    let free_id = (1..=u16::MAX)
        .find(|x| !existing_ids.contains(x))
        .context("Failed to find a free ID. All 65535 IDs are already taken.")?;

    let id_file_path = page.dir.join(format!("{}.id", free_id));
    
    // Create an empty file to reserve the ID
    std::fs::File::create(&id_file_path)
        .with_context(|| format!("Failed to create ID file: {}", id_file_path.display()))?;

    existing_ids.insert(free_id);
    page.id = Some(free_id);
    
    println!("Assigned new ID {} to page: {}", free_id, page.dir.display());
    Ok(())
}







/// Contains the main logic of the static site builder.
fn app_run() -> Result<()> {
    let args = Args::parse();
    
    // Get all tech directories (e.g., 'pages/rust', 'pages/python')
    let techs = get_direct_subdirs(Path::new(ROOT_SRC_DIR))?;

    let mut existing_ids = HashSet::<u16>::new();
    let mut all_pages = Vec::<Page>::new();

    // Collect all pages and existing IDs
    for tech_dir in techs {
        let page_dirs = get_direct_subdirs(&tech_dir)
            .with_context(|| format!("Failed to read pages inside tech directory: {}", tech_dir.display()))?;
        
        for page_dir in page_dirs {
            let p = Page::from_path(&page_dir).with_context(|| format!("Failed to parse page '{}'",page_dir.display()))?;

            if let Some(i) = p.id {
                if !existing_ids.insert(i) {
                    bail!(
                        "Invalid structure: Duplicate ID found. ID {} is already taken by another page.",
                        i
                    )
                }
            }
            all_pages.push(p);
        }
    }
    
    if args.check {
        return Ok(())
    }
    
    
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;
    copy(SITE_RSC_SRC_DIR, SITE_RSC_DIST_DIR, &options)?;  

    // Assign missing IDs
    for page in all_pages.iter_mut() {
        if page.id.is_none() {
            find_and_assign_id(page, &mut existing_ids)?;
        }
    }
    
    // Build all pages
    println!("Starting page build process...");
    for page in all_pages.iter() {
        create_html_page(page)
            .with_context(|| format!("Failed to build page with ID: {}", page.id.map_or_else(|| "N/A".to_string(), |id| id.to_string())))?;
    }
    
    
    // Write indexation.js
    let indexation: Vec<Value> = all_pages
        .iter()
        .map(|p| p.to_json())
        .collect::<Result<Vec<_>>>()?;

    let content = format!("const indexation = {};", serde_json::to_string_pretty(&indexation)?);
    
    let indexation_path = PathBuf::from(ROOT_DIST_DIR).join("indexation.js");
    fs::write(&indexation_path, content)
        .with_context(|| format!("Failed to write HTML file to: {}", indexation_path.display()))?;
    
    println!("Build complete. Total pages built: {}", all_pages.len());

    Ok(())
}

fn main() {
    match app_run() {
        Ok(_) => {
            println!("Ok")
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}
