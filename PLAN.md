# Bloginator Plan

## Configuration

Users should have 2 methods of specifying config options:

1. Command line flags
	- Take precedence over config file options
	- Ex: `bloginator -o build --verbose`

2. Config file
	- By default searched for at `./bloginator.toml`, but may be specified with the `-c <path>` flag
	- Ex:

```toml
output = "build"
verbose = true
```

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
 	- [ ] Move index.html to output folder
 	- [ ] Compile each post into html
 	- [ ] Insert compiled html into the post template
 	- [ ] Create a page for each post -> build/posts
 	- [ ] Copy all files in assets to build/assets (if they don't already exist)
- [ ] Flesh out a little more
 	- [ ] Store metadata alongside posts (title, time/date created, etc.)
 	- [ ] index.html templates for creating a post list
- [ ] Even further
 	- [ ] Minify created html files
 	- [ ] Automatically minify asset files (only ones that are minifiable)
