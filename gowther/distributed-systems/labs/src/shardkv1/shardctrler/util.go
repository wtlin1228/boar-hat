package shardctrler

const Debug = false

func equalUnordered(a, b []string) bool {
	if len(a) != len(b) {
		return false
	}

	set := make(map[string]struct{})

	for _, s := range a {
		set[s] = struct{}{}
	}

	for _, s := range b {
		if _, ok := set[s]; !ok {
			return false
		}
	}

	return true
}
