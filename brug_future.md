# Two more future steps regrading to Brug:

## 1.unified memory with different allocation preferences

The idea behind this idea is that in inside just the old brug project, we have to use different memory allocator and switch between but underlying memory space are separated by different allocators. Which is not the ideal case. The better idea is to use single memory backend and adopt different memory allocators just as different policies.

## 2. Shrinking the page metadata

This idea goes further as currently each page need some sort of metadata section which cost some spaces, but if we keep allocation with the fixed sizes, eliminating (at least) part of those metadata can be saved with some tricks.

## 3. Step forward

In certain case, this two may can be brought together so the memory will be claimed through a specialized driver after the machine start up. Then during the execution, different applications just ask for memory sizes from this device with a value of sizes and a memory chunk returned without a lot of metadata heads. And within these memory chunks the allocation follows different memory allocation polices under different demands.
