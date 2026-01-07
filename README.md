# The Puzzle

Each character in the hiring manager’s email was converted to its Unicode integer representation and then XOR’d with a secret key.  
The key is the **squawk code for a generic emergency**, with leading and trailing zeros removed.

---

## Step 1 — Finding the Secret Key

In aviation, the standard squawk code for a **generic emergency** is: 7700

Removing leading and trailing zeros: 77

So the XOR Key is: 77

## Step 2 — Decoding

Since applying same XOR Coefficient returns to the original char code

`x ^ 77 ^ 77 = x`

Iterate over all elements in u32 bytearray, XOR using same coefficient, and represent as String

`[35,44,47,36,33,13,44,36,63,62,61,44,46,40,96,36,35,57,40,33,33,36,42,40,35,46,40,99,46,34,32]`

Decoded version

"nabil@airspace-intelligence.com"