# HASHERLIFE

*Hasherlife* is a rust implementation of the **hashlife algorithm** more formally known as **Gosper's algorithm**
for **Conway's Game of Life**.

## TODO

### Features

- [x] implement actual hashlife (use IDs / handles as keys into HashMap -> pass around IDs)
- [x] use references instead of IDs / handles (lifetime problems...)

- [ ] more sophisticated RLE pattern reader
- [ ] implement graphical renderer
- [ ] integrate some patterns from <https://conwaylife.com/wiki> like "Kok's Galaxy" or a "Unitcell"
- [ ] create CLI using `clap`

### Refactoring

- [ ] refactor evolution similar to `refs/java/HashLifeTreeNode.java`
- [ ] refactor Leaves to be on level 3 and be represented as Bool8x8 or u64 resp.

- [ ] use references instead of handles (IDs) and a hashset instead of a hashmap (is this even possible?)
