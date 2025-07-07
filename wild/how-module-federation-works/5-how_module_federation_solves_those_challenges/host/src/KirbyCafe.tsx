import { lazy, Suspense } from "react";

const Kirby = lazy(async () => {
  const { Exposed } = await import("kirby");
  return {
    default: Exposed,
  };
});

export const KirbyCafe = () => {
  return (
    <Suspense>
      <Kirby />
    </Suspense>
  );
};
