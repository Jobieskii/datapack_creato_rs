# datapack_creato_rs [wt]

GUI program for creating minecraft datapacks using a node based UI.

**Currently not yet useable, but can be tried out by compiling with rust.**

`$ cargo run`

# Usage
Simply launch the application, open a folder path, and start creating. The different '.json' files are sorted based on type, 
namespace and name on the left, where you can create new files with the `[ + ]` button (note, they are not saved until `save all` 
is pressed). 

Each file type has a different output node which is the default node that generates and it cannot be deleted, 
anything not connected to it will not be saved. 

Press right click to add a new node and connect it's output to another node's input.

Launching with path as the first commandline argument opens that path automatically. 

# Todo
- [ ] Adding every node type
  - prioritizing worldgen as this is my main focus
- [ ] Smarter auto allignment of nodes
- [ ] Proper error messaging
- [ ] undo / redo – the horror
- [ ] visualizations for worldgen
- [ ] Saving the project to some intermediary format that remembers locations of all nodes, non connected nodes and other things that are not relevant in the final datapack
