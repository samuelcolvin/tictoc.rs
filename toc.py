#!/usr/bin/env python3
import sys
from time import time
if len(sys.argv) == 2:
    tic = float(sys.argv[1])
    diff = time() - tic
    if diff < 1:
        s = '{:0.0f}ms'.format(diff * 1000)
    elif diff < 100:
        s = '{:0.2f}s'.format(diff)
    else:
        s = '{:0.0f}s'.format(diff)
    print('{:>5}'.format(s))
