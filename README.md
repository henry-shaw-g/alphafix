# alphafix

Tool to bleed opaque pixel color into transparent pixels.
### Installation

The package can be installed and built to a binary with cargo
```sh
cargo install alphafix
```
TODO: complete release workflow with github action to provide archive binaries for tools like Aftman.  
Alphafix can be built from source by cloning the repository and building using a rust toolchain.

### Usage

Call alphafix and provide paths to the images you want to be modified. The output images by default will be appended with \_fixed
```sh
alphafix resources/blue_circle.png resources/green_triangle.png
```
TODO: support for glob file references

#### Options
`--dir=<path>` : specify target directory where the output images will be stored  
`--append=<str>` : name to append to file name output images, will override the default _fixed  
`--auto` : run command automatically, with no user input (for automated processes). By default, the program does prompt on exit to support drag and drop 
