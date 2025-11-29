# Closure

When the Go compiler analyzes this code, it sees that `counter` is referenced not only in the outer function but also inside the nested `inner` function. Because `inner` may outlive `outer`, the compiler _escapes_ `counter` to the heap instead of storing it on the stack.

This escape to the heap is what allows the returned `inner` function to continue accessing and modifying `counter` even after `outer` has returned.

```go
import "fmt"

func outer() func() {
    counter := 0

    return func() {
        if counter >= 3 {
            return
        }
        counter++
        fmt.Printf("counter %d\n", counter)
    }
}

func main() {
    inner := outer()

    for i := 0; i < 10; i++ {
        inner()
    }
}
```
