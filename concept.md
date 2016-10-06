# What?
* n object files => a running process
  * a linker or a supervisor-like framework
* reloading one or more object files without stopping the process
  * a pure linker would make this hardish
    * we'd need to find everything from the object file, remap the
      address space to reflect the changes and fix pointers
  * a supervisor framework would `mmap` necessary regions and release
    the maps upon replacement. Managing pointers would still be needed
    * how do we want to keep track of things in general?
      * stack/heap
      * integration in `libc` and other dynamically linked components
        * passthrough?
