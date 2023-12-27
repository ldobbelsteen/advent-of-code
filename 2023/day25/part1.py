import random


def find(parents: dict[str, str], i: str):
    r = i
    while r in parents:
        r = parents[r]
    while i in parents:
        p = parents[i]
        parents[i] = r
        i = p
    return i


# Based on https://stackoverflow.com/questions/63771028/minimum-cutkarger-s-algorithm
def karger_min_cut(n: int, edge_set: set[tuple[str, str]]) -> set[tuple[str, str]]:
    edges = list(edge_set)
    random.shuffle(edges)

    parents: dict[str, str] = {}
    for source, target in edges:
        if n <= 2:
            break
        s = find(parents, source)
        t = find(parents, target)
        if s == t:
            continue
        parents[s] = t
        n -= 1

    return set((s, t) for s, t in edges if find(parents, s) != find(parents, t))


def main():
    with open("input.txt", "r") as file:
        vertices: set[str] = set()
        edges: set[tuple[str, str]] = set()

        for line in file.readlines():
            line = line.strip()
            source, targets = line.split(": ")
            vertices.add(source)
            for target in targets.split(" "):
                vertices.add(target)
                if source <= target:
                    edges.add((source, target))
                else:
                    edges.add((target, source))

        cut = None
        n = len(vertices)
        while cut is None or len(cut) > 3:
            cut = karger_min_cut(n, edges)

        remaining_edges = list(edges.difference(cut))
        left_start, right_start = list(cut)[0]
        left: set[str] = set([left_start])
        right: set[str] = set([right_start])
        while len(left) + len(right) != n:
            for edge in remaining_edges:
                if edge[0] in left:
                    left.add(edge[1])
                if edge[1] in left:
                    left.add(edge[0])
                if edge[0] in right:
                    right.add(edge[1])
                if edge[1] in right:
                    right.add(edge[0])

        print(len(left) * len(right))


if __name__ == "__main__":
    main()
