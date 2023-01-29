# Usage

## Bleeding into transparent pixels
```sh
alphafix <image paths ...>
```
Edits the given images so that opaque pixels and their transparent neighbors have similar colors. Addresses the issue of wierd color sampling in engines such as Roblox.

## Flags
### opaque
```sh
alphafix <image paths ...> --opaque
```
After bleeding the images, sets the pixels that were entirely transparent to opaque. All output images will be copied and named "filepath_opaque.png". Useful if some images still are being sampled undesirably or for debugging.