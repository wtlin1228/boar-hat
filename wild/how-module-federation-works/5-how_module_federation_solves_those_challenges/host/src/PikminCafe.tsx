import { lazy, Suspense } from "react";

const Pikmin = lazy(async () => {
  const { Exposed } = await import("pikmin");
  return {
    default: Exposed,
  };
});

export const PikminCafe = () => {
  return (
    <Suspense>
      <Pikmin />
    </Suspense>
  );
};
