### Unreleased (Next Candidate)
* Organize the ui module
* Add basic 2D Perlin Noise height map

### v0.2.5 [June 4, 2020]
* Automatic chunk loading (very slow; unoptimized)
    * Threading is difficult so I'll refactor the whole thing under a new repository (TheEndlessWorld)

### v0.2.4 [June 3, 2020]
* Removed sector and changed chunk side to 64x64x64

### v0.2.3 [June 2, 2020]
* Fully functioning FPS reader
* Also includes a working text widget (with limited characters [A-Z][0-9][ ])

### v0.2.2 [May 31, 2020]
* Documented the file structure
    * hopefully attracts some contributors later :)

### v0.2.1 [May 30, 2020]
* Added basic UI support
    * Currently in Alpha phase
* Fixed the windowing issue for the main world renderer

### v0.2.0 [May 20, 2020]
* Chunks now optimized with transparent blocks
* Added [T] key to escape the mouse lock (temp.)

### v0.1.0 [May 18, 2020]
* World is now rendered by chunks instead of each discrete block type.
* Each chunk owns a block data, which gets converted into vertices and indices for rendering
    * a lot of GPU optimization here: index rendering, smaller vertex data types, etc.
* Each chunk has its own block data, which gets called to render in conjunction with the parameter of the owner of mesh struct.
