// D -> [Default from "./other/file/path"]
// E -> [E from "./other/file/path"]
// LocalF -> [F from "./other/file/path"]
import D, { E, F as LocalF } from "./other/file/path";

// localObj -> [D, E, LocalF]
const localObj = {
  // access Object by path only counted as Object itself
  a: D.Header,
  b: E.Body,
  c: LocalF.Boo,
  x: {
    y: {
      z: () => D.Boo,
    },
  },
};

// localArr -> [D, E, localObj]
const localArr = [D, E, localObj];

// LocalE -> [E, LocalF]
const LocalE = () => {
  const a = 1;
  const b = 2;
  const D = "I'm not the <default export> from <other file path> ";
  return (
    <div>
      <E a={a} b={b} c="c" d={D}>
        <article>
          <span>
            <E>
              <LocalF />
            </E>
          </span>
        </article>
      </E>
    </div>
  );
};

// Foo -> [D, LocalE, localObj, localArr]
const Foo = () => {
  return (
    <D>
      {/* access Object by path only counted as Object itself */}
      {/* access Array by index only counted as Array itself */}
      <LocalE a={localObj.x.y.z} b={localArr[2]} />
    </D>
  );
};

// Default -> Foo
export default Foo;
