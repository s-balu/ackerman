# import numpy as np
# import numba as nb
import sys

sys.setrecursionlimit(10_000_000)


def ackermann(m, n):
    if m == 0:
        ans = n + 1

    elif n == 0:
        ans = ackermann(m - 1, 1)

    else:
        ans = ackermann(m - 1, ackermann(m, n - 1))

    return ans


if __name__ == "__main__":
    m = int(sys.argv[1])
    n = int(sys.argv[2])
    print(f"Acackermann({m}, {n}): ", ackermann(m, n))
