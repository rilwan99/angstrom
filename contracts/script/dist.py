from random import randint
from collections import Counter

SIZE = 16


xs = []
ys = []

for _ in range(1_000_000):
    # m = randint(0, SIZE)
    # x = randint(0, m)
    # y = randint(m, SIZE)
    x = randint(0, SIZE)
    y = randint(x, SIZE)
    xs.append(x)
    ys.append(y)


def report(l: list[int]):
    avg = sum(l) / len(l)
    print(f'  avg: {avg:.3f}')
    c = Counter(l)
    counts = sorted(c.items(), key=lambda d: d[0])
    for v, t in counts:
        print(f'  {v}: {t/len(l):.2%}')


print('x:')
report(xs)
print('y:')
report(ys)
