import * as React from "react";

function useMatchMedia(query: string) {
  const [matches, setMatches] = React.useState<boolean>();

  React.useEffect(() => {
    function handleChange(ev: MediaQueryListEvent) {
      setMatches(ev.matches);
    }

    const mediaQueryList = window.matchMedia(query);
    mediaQueryList.addEventListener("change", handleChange);
    setMatches(mediaQueryList.matches);

    return () => {
      mediaQueryList.removeEventListener("change", handleChange);
    };
  }, [query]);

  return matches;
}

export function useScreens() {
  const sm = useMatchMedia("(min-width: 640px)");
  const md = useMatchMedia("(min-width: 768px)");
  const lg = useMatchMedia("(min-width: 1024px)");
  const xl = useMatchMedia("(min-width: 1280px)");
  const xxl = useMatchMedia("(min-width: 1536px)");

  return React.useMemo(
    () => ({
      sm,
      md,
      lg,
      xl,
      xxl,
    }),
    [sm, md, lg, xl, xxl]
  );
}
