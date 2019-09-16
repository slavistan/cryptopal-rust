Defining a measure between bitsets
==================================

Let the **Hamming-mass **μ** of a byte *u*** denote its number of set
bits.

**μ**(*u*) := Number of set bits in *u*  ∀*u* ∈ **U**<sub>**N**</sub>

Further extend this notion to let the **Hamming-mass of a byteset** be
the total number of set bits over the whole set.

$$
\\mathbf{\\mu}(U) := \\sum \\limits\_{u \\in U} \\mathbf{\\mu}(u)
$$

Let the **Hamming-density of a byteset** be the average number of set
bits.

$$
\\mathbf{\\rho}(U) := \\frac{1}{\\|U\\|} \\cdot \\mathbf{\\mu}(U) \\,\\,\\forall\\,\\, \\{\\} \\neq U \\subseteq \\mathbf{U\_N}
$$

TODO FROM HERE
==============

Let the **Hamming-distance between two bytesets** be defined as the
average number of bits that differ between any two members of the
subsets.

$$
\\text{for any } V,W \\subseteq \\mathbf{U\_N}\\,\\text{ let }\\, H(V, W) := \\frac{1}{\\|V \\times W \\|} \\sum \\limits\_{v \\in V, w \\in W} H(v, w) = \\frac{1}{\\|V\\| \\cdot \\|W\\|} \\sum \\limits\_{v \\in V, w \\in W} H(v \\mathbf{\\,\\oplus\\,}w)
$$

The Hamming-distance of any byteset with respect to itself is less than
the Hamming-distance between any two differing bytesets.

$$ U, V, W
