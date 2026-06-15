# Bloginator Plan

## Specifying Configuration

Users should have 2 methods of specifying config options:

1. Command line flags
	- Take precedence over config file options
	- Ex: `bloginator build -o ./build --verbose`

2. Config file
	- By default searched for at `./bloginator.toml`, but may be specified with the `-c <path>` flag
	- Ex:

```toml
output_folder = "build"
verbose = true
```

Information about configuration options can be found in CONFIG.md

## File Strucure

```text
├─bloginator.toml -> Config file
├─index.html ------> Blog homepage
├─post.html -------> Template for each post
├─posts/ ----------> Contains a markdown file for each post
├─assets/ ---------> All files within will be copied to build/assets
└─build/ ----------> Default directory containing the built blog
```

## Todo

- [ ] Basic functionality
	- [x] Set up clap
		- [x] Create the build subcommand
		- [x] Create the -c and -v flags
	- [x] Integrate clap with config
		- Clap should override config file options
	- [x] Move index.html to output folder
	- [x] Compile each post into html
	- [x] Insert compiled html into the post template
	- [x] Create a page for each post -> build/posts
	- [ ] Copy all files in assets to build/assets (if they don't already exist)
- [ ] Flesh out a little more
	- [ ] Store metadata alongside posts (title, time/date created, etc.)
	- [ ] Create the create-post subcommand
		- Initializes a new md post along with its metadata
	- [ ] index.html templates for creating a post list
- [ ] Even further
	- [ ] Minify created html files
	- [ ] Automatically minify asset files (only ones that are minifiable)
