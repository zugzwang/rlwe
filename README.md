# RLWE

This crate implements arithmetic in cyclotomic rings used in
Ring-Learning With Errors cryptography, namely, specific rings of the form
`K[X] / (phi_{2n}(X))` where `K` is a field and `phi_{2n}` is the `2n`-th
cyclotomic polynomial for `n` a power of 2.

For `n` a power of 2 (e.g., `phi_{2n}(X) = Xⁿ+1`), and `p, q` primes such that
`p ≡ q ≡ 1 mod 2n`, `rlwe` implements arithmetic on the so-called _negacyclic_
rings:

```
                       ℤ[X]
        R     :=      ------ ,
                      (Xⁿ+1)

                      ℤ_p[X]
       R_p    :=     ------- ,
                      (Xⁿ+1)

                    ℤ_{pq^l}[X]
     R_{pq^l} :=   ------------- .
                       (Xⁿ+1)
```
