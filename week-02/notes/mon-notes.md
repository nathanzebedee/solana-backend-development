# 10/31/2022

### memory - heap and stack
**stack**:
* the simplest place to store data
* data stored on stack must have known, fixed size at compile time
* copying items on the stack is fast and (memory) cheap
**heap**:
* used to store more complex, dynamic data structures
* data stored on heap can have unknown size at compile time
* we typically do not want to copy data on the heap - it is very expensive

### clearing memory
**garbage collection**:
* newer languages (python, go, etc) have background programs that automatically keep track of and remove items from the stack and heap that are no longer being used.
    * pros:
        * easy to use
        * no need to worry about memory management
    * cons:
        * programs are slower
        * unable to make optimizations for memory allocation
**manual memory management**:
* older languages (c, c++, etc) require you to manually keep track of the memory you have been allocated and free it when you are done with it.
    * pros:
        * control over memory management (for optimizations)
    * cons:
        * can be hard to use
        * very easy to make critical mistakes
**ownership**:
* rust takes a novel approach to memory management, which we call "ownership."