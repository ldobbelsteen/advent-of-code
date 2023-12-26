import networkx as nx


def main():
    with open("input.txt", "r") as file:
        G: nx.Graph = nx.Graph()
        for line in file.readlines():
            line = line.strip()
            source, targets = line.split(": ")
            for target in targets.split(" "):
                G.add_edge(source, target, capacity=1)

        nodes = G.nodes()
        for source in nodes:
            for target in nodes:
                if source != target:
                    size, (left, right) = nx.minimum_cut(G, source, target)
                    if size <= 3:
                        print(len(left) * len(right))
                        return


if __name__ == "__main__":
    main()
