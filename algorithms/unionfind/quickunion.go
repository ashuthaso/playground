package unionfind

type QuickUnionUF struct {
	ids []int
}

func (uf *QuickUnionUF) new(size int) {
	uf.ids = make([]int, size)
	for i := 0; i < size; i++ {
		uf.ids[i] = i
	}
}

func (uf *QuickUnionUF) root(i int) int {
	for i != uf.ids[i] {
		i = uf.ids[i]
	}
	return i
}

func (uf *QuickUnionUF) connected(p, q int) bool {
	return uf.root(p) == uf.root(q)
}

func (uf *QuickUnionUF) union(p, q int) {
	pRoot := uf.root(p)
	qRoot := uf.root(q)
	uf.ids[pRoot] = qRoot
}
