Given an image of W,H pixels, where a 2d matrix of all the dark and light colors in the image, where dark color are marked as 1 and light color as 0.

![img](img/Untitled2.png)

find the points where the 2d matrix takes turn. For example, if the matrix is

**input:**
```
00000000
01111000
01111000
00011000
00011000
00000000
00000000
01111000
01111000
00011000
00011000
00000000
```

**output:**

```
wall1: (1,1),(1,5),(4,5),(4,4),(4,3),(2,3),(2,1),(1,1)
wall2: (7,1),(7,5),(10,5),(10,4),(10,3),(8,3),(8,1),(7,1)
```