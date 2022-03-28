# RLWE

This crate implements arithmetic in cyclotomic rings used in
Ring-Learning With Errors cryptography.

For `n` a power of 2 (e.g., `phi_{2n}(X) = Xⁿ+1` is the `2n`-th cyclotomic
polynomial), and `p` a prime such that `p ≡ 1 mod 2n`, `rlwe` implements
arithmetic on the rings

```
                       ℤ[X]
        R     :=      ------ ,
                      (Xⁿ+1)

                      𝔽_p[X]
       R_p    :=     ------- ,
                      (Xⁿ+1)

                    𝔽_{q^l}[X]
     R_{q^l} :=   ------------- .
                       (Xⁿ+1)
```

(WIP)
