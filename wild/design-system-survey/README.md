# Screening Interview for Design Systems

- Owner: Maintained by some big company could be expected to be more stable, performant and well designed.
- License: Can be used commercially?
- Styling: Affect customization, CSS specificity, readability, maintainability, performance, etc.
- Styling Performance: Critical for building large applications.
- Types: Easier to maintain, migrate and develop.
- Figma Kit: Easier to design and make it possible to leverage modern tools like visual-copilot.
- RWD: It’s baseline for modern applications.
- Documentation: Good docs bring good developer experience and productivity.
- Accessibility: It's a requirement in some countries.
- Design System: Good design principle brings good designer experience.
- Community: Where to discuss, propose something and ask questions.
- Issues Closed 2024: Is it still actively maintained in 2024?
- Data Visualization: It’s nice to have.
- Platforms: Maybe someday we wanna have our brand identification cross-platform.
- Open for Contribution: As we are getting more familiar with it, we might wanna contribute back and help fixing some bugs.

|                       | [gestalt][gestalt]             | [polaris][polaris]             | [carbon][carbon]              | [joy-ui][joy-ui]               | [primer][primer]              | [atlassian][atlassian]           | [fluent-ui][fluent-ui]           | [chakra-ui][chakra-ui]    | [grommet][grommet]      | [flowbite][flowbite]      | [shadcn-ui][shadcn-ui]      | [material-ui][material-ui]          | [mantine][mantine]       |
| --------------------- | ------------------------------ | ------------------------------ | ----------------------------- | ------------------------------ | ----------------------------- | -------------------------------- | -------------------------------- | ------------------------- | ----------------------- | ------------------------- | --------------------------- | ----------------------------------- | ------------------------ |
| Owner                 | Pinterest                      | Shopify                        | IBM                           | MUI                            | GitHub                        | Atlassian                        | Microsoft                        | Community                 | Community               | Themesberg                | Community                   | MUI                                 | Community                |
| License               | Apache-2.0                     | ❌ [Ref][polaris lic]          | Apache-2.0                    | MIT                            | MIT                           | Apache-2.0                       | MIT                              | MIT                       | Apache-2.0              | MIT                       | MIT                         | MIT                                 | MIT                      |
| Styling               | CSS module                     | CSS module                     | SCSS                          | CSS-in-JS                      | CSS-in-JS                     | CSS-in-JS                        | CSS module                       | CSS-in-JS                 | CSS-in-JS               | Tailwind CSS              | Tailwind CSS                | CSS-in-JS                           | CSS module               |
| Styling Performance   | ✅ Plain CSS                   | ✅ Plain CSS                   | ✅ Plain CSS                  | ⚠️ [Ref][joy-ui perf]          | ❌ [Ref][primer perf]         | ❌                               | ✅ Plain CSS                     | ❌ [Ref][chakra-ui perf]  | ❌                      | ✅ Plain CSS              | ✅ Plain CSS                | ⚠️ [Ref][material-ui perf]          | ✅ Plain CSS             |
| Types                 | ✅                             | ✅                             | ✅                            | ✅                             | ✅                            | ✅                               | ✅                               | ✅                        | ✅                      | ✅                        | ✅                          | ✅                                  | ✅                       |
| Figma Kit             | ❌ [Ref][gestalt fk]           | ✅ [Ref][polaris fk]           | ✅ [Ref][carbon fk]           | ✅ [Ref][joy-ui fk]            | ✅ [Ref][primer fk]           | ✅ [Ref][atlassian fk]           | ✅ [Ref][fluent-ui fk]           | ✅ [Ref][chakra-ui fk]    | ❌                      | ✅ [Ref][flowbite fk]     | ✅ [Ref][shadcn-ui fk]      | ✅ [Ref][material-ui fk]            | ✅ [Ref][mantine fk]     |
| RWD                   | ✅                             | ✅                             | ✅                            | ✅                             | ✅                            | ✅                               | ✅                               | ✅                        | ✅                      | ✅                        | ✅                          | ✅                                  | ✅                       |
| Documentation         | ✅ [Ref][gestalt doc]          | ✅ [Ref][polaris doc]          | ✅ [Ref][carbon doc]          | ✅ [Ref][joy-ui doc]           | ✅ [Ref][primer doc]          | ✅ [Ref][atlassian doc]          | ✅ [Ref][fluent-ui doc]          | ✅ [Ref][chakra-ui doc]   | ✅ [Ref][grommet doc]   | ✅ [Ref][flowbite doc]    | ✅ [Ref][shadcn-ui doc]     | ✅ [Ref][material-ui doc]           | ✅ [Ref][mantine doc]    |
| Accessibility         | ✅ [WCAG 2.2 AA][gestalt a11y] | ✅ [WCAG 2.1 AA][polaris a11y] | ✅ [WCAG 2.2 AA][carbon a11y] | ✅ [WAI-ARIA 1.2][joy-ui a11y] | ✅ [WCAG 2.1 AA][primer a11y] | ✅ [WCAG 2.2 AA][atlassian a11y] | ✅ [WCAG 2.1 AA][fluent-ui a11y] | ﹖                        | ﹖                      | ﹖                        | ✅ [Ref][shadcn-ui a11y]    | ✅ [WAI-ARIA 1.2][material-ui a11y] | ﹖                       |
| Design System         | ✅ [Ref][gestalt ds]           | ✅ [Ref][polaris ds]           | ✅ [Ref][carbon ds]           | ﹖                             | ✅ [Ref][primer ds]           | ✅ [Ref][atlassian ds]           | ✅ [Ref][fluent-ui ds]           | ✅ [Ref][chakra-ui ds]    | ﹖                      | ❌                        | ❌                          | ✅ [Ref][material-ui ds]            | ❌                       |
| Community             | ❌                             | ✅ [Ref][polaris com]          | ✅ [Ref][carbon com]          | ✅ [Ref][joy-ui com]           | ✅ [Ref][primer com]          | ❌                               | ✅ [Ref][fluent-ui com]          | ✅ [Ref][chakra-ui com]   | ✅ [Ref][grommet com]   | ✅ [Ref][fluent-ui com]   | ✅ [Ref][shadcn-ui com]     | ✅ [Ref][material-ui com]           | ✅ [Ref][mantine com]    |
| Issues Closed 2024    | ﹖                             | 277 [Ref][polaris issue]       | 631 [Ref][carbon issue]       | 496 [Ref][joy-ui issue]        | 88 [Ref][primer issue]        | ﹖                               | 696 [Ref][fluent-ui issue]       | 99 [Ref][chakra-ui issue] | 57 [Ref][grommet issue] | 122 [Ref][flowbite issue] | 1199 [Ref][shadcn-ui issue] | 496 [Ref][material-ui issue]        | 461 [Ref][mantine issue] |
| Data Visualization    | ✅ [Ref][gestalt dv]           | ❌                             | ✅ [Ref][carbon dv]           | ✅ [Ref][joy-ui dv]            | ❌                            | ❌                               | ❌                               | ❌                        | ✅ [Ref][grommet dv]    | ✅ [Ref][flowbite dv]     | ✅ [Ref][shadcn-ui dv]      | ✅ [Ref][material-ui dv]            | ✅ [Ref][mantine dv]     |
| Platforms             | Web, IOS, Andriod              | Web                            | Web                           | Web                            | Web                           | Web                              | Web                              | Web                       | Web                     | Web                       | Web                         | Web, IOS, Andriod                   | Web                      |
| Open for Contribution | ⚠️ Limited                     | ✅ [Ref][polaris con]          | ✅ [Ref][carbon con]          | ✅ [Ref][joy-ui con]           | ✅ [Ref][primer con]          | ❌ [Ref][atlassian con]          | ✅ [Ref][fluent-ui con]          | ✅ [Ref][chakra-ui con]   | ✅ [Ref][grommet con]   | ✅ [Ref][flowbite con]    | ✅ [Ref][shadcn-ui con]     | ✅ [Ref][material-ui con]           | ✅ [Ref][mantine con]    |

## Screening Result

- gestalt: ❌ Figma Kit
- polaris: ❌ License
- carbon: ✅
- joy-ui: ✅
- primer: ❌ Styling Performance
- atlassian: ❌ Styling Performance
- fluent-ui: ✅
- chakra-ui: ❌ Styling Performance
- grommet: ❌ Styling Performance
- flowbite: ✅
- shadcn-ui: ✅
- mui: ✅
- mantine: ✅
- semi-ui
- next-ui

# [fluent-ui][fluent-ui]

- 🙃 The HTML structure is not that simple compared to `material-ui`.
- 🫡 Have good styling performance by developing its own zero-runtime CSS-in-JS library `Griffel`
- 💯 Reduce the re-renders caused by contexts by using `useContextSelector()`
- 🌱 Provide consistent developer experience by following some patterns like the Trigger Pattern
- 😍 Very easy to customize components with `Slot` API, styles will be merged with the `mergeClasses` API

  ```tsx
  import { Button, makeStyles, tokens } from "@fluentui/react-components";
  import { CalendarMonthRegular } from "@fluentui/react-icons";

  const useStyles = makeStyles({
    rowWith2Cols: {
      display: "flex",
      justifyContent: "space-around",
    },
    button: {
      border: `2px dashed ${tokens.colorPaletteLavenderBorderActive}`,
      color: tokens.colorPaletteLavenderBorderActive,
    },
    buttonIcon: {
      color: tokens.colorPaletteLavenderBorderActive,
    },
  });

  const OverrideButtonStyle = () => {
    const styles = useStyles();

    return (
      <div className={styles.rowWith2Cols}>
        <Button icon={<CalendarMonthRegular />}>Default</Button>

        <Button
          className={styles.button}
          icon={{
            className: styles.buttonIcon,
            children: <CalendarMonthRegular />,
          }}
        >
          Default
        </Button>
      </div>
    );
  };
  ```

# [flowbite][flowbite]

- 🚨 Have serious performance issue: https://github.com/themesberg/flowbite-react/issues/1447
- 🍃 Tailwind based
- ⚙️ More like a tool for building design system
- 🧰 Have many handy blocks ready to be copied and pasted: https://flowbite.com/blocks/
- 😋 Flexible to customize component, but not so easy since we need to look into the source code to find out which parts of the theme will be applied to which part of the component

  ```
  The default structure for button:

  <button> <- base,
  |           disabled,
  |           color,
  |           gradientDuoTone,
  |           gradient,
  |           outline.color,
  |           pill,
  |           fullSized
  |
  |   <span> <- inner.base,
  |   |         outline,
  |   |         outline.pill,
  |   |         size,
  |   |         inner.outline,
  |   |         isProcessing,
  |   |         inner.isProcessingPadding,
  |   |         inner.position
  |   |
  |   |   <span></span> <- spinnerSlot,
  |   |                    spinnerLeftPosition
  |   |
  |   |   <span></span> <- label
  |   |
  |   </span>
  |
  </button>
  ```

  ```tsx
  import * as React from "react";
  import type { CustomFlowbiteTheme } from "flowbite-react";
  import { Button as FlowbiteButton, Spinner } from "flowbite-react";

  const customTheme: CustomFlowbiteTheme["button"] = {
    color: {
      rose: "bg-rose-500 hover:bg-rose-600", // "rose" is a new variant
    },
    outline: {
      color: {
        // need to look into the source code to find out how to customize
        // the outline color for our new color variant "rose"
        rose: "border border-8 border-rose-300 hover:border-rose-800",
      },
    },
    // label style here is tightly bound to the "rose" variant
    label: "text-rose-500 group-hover:text-white",
  };

  export function Button() {
    const [isProcessing, setIsProcessing] = React.useState(false);

    return (
      <FlowbiteButton
        theme={customTheme}
        label="Click me"
        color="rose"
        fullSized
        isProcessing={isProcessing}
        processingLabel="Processing..."
        processingSpinner={<Spinner size="lg" />}
        outline
        pill
        size="lg"
        onClick={() => {
          setIsProcessing(!isProcessing);
        }}
      />
    );
  }
  ```

# [shadcn/ui][shadcn-ui]

- 🍃 Tailwind + Radix Primitives
- 🧰 Have many handy blocks ready to be copied and pasted: https://ui.shadcn.com/blocks
- 💡 Not a component library, it's a reference to build our own component libraries
- 🍺 Designed with `class-variance-authority`: https://cva.style/docs
- 😍 Super easy to customize component

  `$ npx shadcn-ui@latest add button`

  ```tsx
  import * as React from "react";
  import { Slot } from "@radix-ui/react-slot";
  import { cva, type VariantProps } from "class-variance-authority";

  import { cn } from "@/lib/utils";

  const buttonVariants = cva(
    "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
    {
      variants: {
        variant: {
          default: "bg-primary text-primary-foreground hover:bg-primary/90",
          destructive:
            "bg-destructive text-destructive-foreground hover:bg-destructive/90",
          outline:
            "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
          secondary:
            "bg-secondary text-secondary-foreground hover:bg-secondary/80",
          ghost: "hover:bg-accent hover:text-accent-foreground",
          link: "text-primary underline-offset-4 hover:underline",
        },
        size: {
          default: "h-10 px-4 py-2",
          sm: "h-9 rounded-md px-3",
          lg: "h-11 rounded-md px-8",
          icon: "h-10 w-10",
        },
      },
      defaultVariants: {
        variant: "default",
        size: "default",
      },
    }
  );

  export interface ButtonProps
    extends React.ButtonHTMLAttributes<HTMLButtonElement>,
      VariantProps<typeof buttonVariants> {
    asChild?: boolean;
  }

  const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
    ({ className, variant, size, asChild = false, ...props }, ref) => {
      const Comp = asChild ? Slot : "button";
      return (
        <Comp
          className={cn(buttonVariants({ variant, size, className }))}
          ref={ref}
          {...props}
        />
      );
    }
  );
  Button.displayName = "Button";

  export { Button, buttonVariants };
  ```

# Styling

A design system provides the base styles, but sometimes we still want to override some components.

## Static CSS

It's fast, but has CSS specificity issues.

```css
/* base-styles.css */
.ui.vertical.menu {
  color: #ccc;
  margin: 0;
}

/* overrides.css */
.menu {
  margin: 5px;
} /* ❌, not enough specific */
.ui.menu {
  margin: 5px;
} /* ❌, not enough specific */

.ui.vertical.menu {
  margin: 5px;
} /* ✅, that works */
#foo-menu {
  margin: 5px;
} /* ✅, that works, too */
```

CSS needs help from more high-level tools and techniques, such as CSS-in-JS, or predecessors to make it closer to a programmer language.

## BEM (Block Element Modifier)

BEM is a set of rules for naming CSS classes, but everything is a single class and nothing is nested. It's recommended to avoid combine selectors, and as a design system, we can follow this rule. The most important thing about this approach is to ensure that developers will follow it. We can use some lint rules to enforce that, but that's not 100% guaranteed. And also there's no guarantee that a developer from team A won't use the same name from team B.

```css
.button {
  color: #ccc;
}
.button.primary {
  color: #fff;
}

/* BEM approach */
.button {
  color: #ccc;
}
.button--primary {
  color: #fff;
}
```

## CSS modules

CSS modules solves the naming problem by assigning random name through CSS class. All class names and animation names are scoped locally by default. CSS modules perfectly solve the naming problem but unfortunately not the specificity problem. So the common practice is to combine the CSS modules and BEM. This works for simple cases. For more complex ones, it does not, as selectors could be combined.

```css
/* "MODULE" represents hash produced */
/* by CSS modules */

/* Combined selectors */
.MODULE__button.MODULE__primary {
  color: #fff;
}

/* ⬇️ To a single selector with BEM */
.MODULE__button--primary {
  color: #fff;
}
```

## CSS-in-JS

This is where CSS-in-JS libraries like `Emotion` enter the game, to offer solutions for specificity problems. Well, at least a few of them. Emotion is blazing fast and very close to plain CSS in terms of performance. So, why cannot we use it?

1. It does a significant amount of runtime work.
2. It outputs monolithic classes.

```css
.base {
  color: red;
  font-size: 32px;
}
.overrides {
  color: blue;
}

/* merge to */
.base-n-overrides {
  color: red;
  font-size: 32px;
  /* 👇 wins based on order */
  color: blue;
}
```

### CSS-in-JS + CSS variables

```jsx
function Backdrop({ opacity, color, children }) {
  return (
    <Wrapper opacity={opacity} color={color}>
      {children}
    </Wrapper>
  );
}
const Wrapper = styled.div`
  opacity: ${(p) => p.opacity};
  background-color: ${(p) => p.color};
`;
```

Whenever these values change, `styled-components` will need to re-generate the class and re-inject it into the document's `styleSheets`, which can be a performance liability in certain cases (eg. doing JS animations).

Here's another way to solve the problem, using CSS variables:

```jsx
function Backdrop({ opacity, color, children }) {
  return (
    <Wrapper
      style={{
        "--color": color,
        "--opacity": opacity,
      }}
    >
      {children}
    </Wrapper>
  );
}
const Wrapper = styled.div`
  opacity: var(--opacity, 0.75);
  background-color: var(--color, var(--color-gray-900));
`;
```

This trick also enables color mode without flashing: https://leonerd.blog/posts/building-a-react-component-library-part1/#enable-color-mode-with-css-custom-properties

## Atomic CSS

A classic example is tailwind: https://tailwindcss.com/

## Zero-runtime CSS-in-JS + Atomic CSS

Combine the benefits of static CSS and CSS-in-JS, so it's fast and has no CSS specificity issues.

- easy to read and maintain
- CSS extraction
- merge classes

```js
const stylesA = {
  paddingLeft: "pl-10",
  color: "c-ref",
};
const styleB = {
  color: "c-blue",
};

/* 1. Merge objects to remove duplicate keys */
const mergedStyles = {
  ...stylesA,
  ...stylesB,
};

/* Get CSS classes */
const classes = Object.values(mergedStyles); // ➡️ ["pl-10", "c-blue"]
```

### Why is class merging so important?

```css
.color-blue {
  color: blue;
}
.color-red {
  color: red;
}
```

```html
<span class="red blue">
  Hello! What color I have?
  <!-- 🍎 -->
</span>
```

Without class merging, the insert order of atomic CSS will affect the style. And in CSS-in-JS world, the insert order is based on the components' render order. So that could cause random behavior, BIG PROBLEM 🚨.

### Why is LVFHA order also important?

Those classes in CSS have the same specificity:

- Link
- Visited
- Focus within
- Focus
- Focus visible
- Hover
- Active

https://codesandbox.io/s/lvfha-puzzle-ihbict

```jsx
import Frame from "react-frame-component";

export default function App() {
  return (
    <div>
      <p>
        Both frame render the same CSS, but order of appearance is different.
      </p>

      <Frame>
        <style>{`.button:hover { color: red;  }`}</style>
        <style>{`.button:focus { color: blue; }`}</style>

        <button className="button">Hello, focus & hover me</button>
      </Frame>
      <br />
      <Frame>
        <style>{`.button:focus { color: blue; }`}</style>
        <style>{`.button:hover { color: red;  }`}</style>

        <button className="button">Hello, focus & hover me</button>
      </Frame>
    </div>
  );
}
```

The solution is to insert these rules into separate style elements (style buckets):

```html
<!-- Catch all other -->
<style data-make-styles-bucket="d" />
<!-- :link -->
<style data-make-styles-bucket="l" />
<!-- :visited -->
<style data-make-styles-bucket="v" />
<!-- :focus-within -->
<style data-make-styles-bucket="w" />
<!-- :focus -->
<style data-make-styles-bucket="f" />
<!-- :focus-visible -->
<style data-make-styles-bucket="i" />
<!-- :hover -->
<style data-make-styles-bucket="h" />
<!-- :active -->
<style data-make-styles-bucket="a" />
<!-- @keyframe -->
<style data-make-styles-bucket="k" />
<!-- @media, @supports, etc. -->
<style data-make-styles-bucket="t" />
```

### Why is shorthand causing problem?

Order of styles inside JavaScript objects is same, but the resulting CSS is different due to rules insertion order 💣

```js
import css from "atomic-css-in-js";

const classNameB = css({
  padding: 5,
  paddingTop: 10,
});

// 👇 Result
// .padding { padding: 5px; }
// .padding-top { padding-top: 10px; }
```

```js
import css from "atomic-css-in-js";

const classNameA = css({
  paddingTop: 10,
});
const classNameB = css({
  padding: 5,
  paddingTop: 10,
});

// 👇 Result, order is changed
// .padding-top { padding-top: 10px; }
// .padding { padding: 5px; }
```

### What people are working on

- MUI team:
  - [Pigment CSS](https://github.com/mui/pigment-css)
  - [\[RFC\] Zero-runtime CSS-in-JS implementation #38137](https://github.com/mui/material-ui/issues/38137)
- Github team:
  - [Moving on from runtime CSS-in-JS at scale - Siddharth Kshetrapal | JSHeroes 2023](https://www.youtube.com/watch?v=OrIuKl_x0vE)
  - [siddharthkp/css-out-js](https://github.com/siddharthkp/css-out-js)
- Chakra team:
  - [Panda](https://github.com/chakra-ui/panda)
  - [If you're not okay with runtime CSS-in-JS, use Ark and Panda](https://www.adebayosegun.com/blog/the-future-of-chakra-ui#what-should-i-use-in-my-project:~:text=If%20you%27re%20not%20okay%20with%20runtime%20CSS%2Din%2DJS%2C%20use%20Ark%20and%20Panda)
- Meta:
  - [StyleX](https://github.com/facebook/stylex)
  - [Building the New Facebook with React and Relay | Frank Yan](https://www.youtube.com/watch?v=9JZHodNR184)
- Microsoft:
  - [Griffel](https://github.com/microsoft/griffel)
  - [Fluent UI React Insights: Griffel](https://learn.microsoft.com/en-us/shows/fluent-ui-insights/fluent-ui-insights-griffel)

# Patterns

## Trigger Pattern

```tsx
<Host>
    <Trigger>
        <Button>A button</Button>
    </Trigger>

    <Popover>
        Hello world!
    </Popover>
</Popover>
```

```tsx
// `getChildren` restricts the ordering of trigger and popover
const getChildren = (props) => {
  const children = React.Children.toArray(
    props.children
  ) as React.ReactElement[];

  let trigger: React.ReactElement | undefined = undefined;
  let popover: React.ReactElement | undefined = undefined;
  if (children.length === 2) {
    trigger = children[0];
    popover = children[1];
  } else if (children.length === 1) {
    popover = children[0];
  }

  return [trigger, popover];
};

const Host = (props) => {
  const [trigger, popover] = getChildren(props);
  const contextValue = useContextValue(props);

  return (
    <ContextProvider value={contextValue}>
      {trigger}
      {popover}
    </ContextProvider>
  );
};
```

<!-- gestalt -->

[gestalt]: https://github.com/pinterest/gestalt
[gestalt fk]: https://gestalt.pinterest.systems/get_started/designers#Private-Figma-plugins
[gestalt dv]: https://gestalt.pinterest.systems/foundations/data_visualization/overview
[gestalt doc]: https://gestalt.pinterest.systems/web/overview
[gestalt a11y]: https://gestalt.pinterest.systems/foundations/accessibility
[gestalt ds]: https://gestalt.pinterest.systems/foundations/overview

<!-- polaris -->

[polaris]: https://github.com/Shopify/polaris
[polaris lic]: https://github.com/Shopify/polaris/blob/main/LICENSE.md
[polaris fk]: https://www.figma.com/community/file/1293611962331823010/polaris-components
[polaris doc]: https://polaris.shopify.com/components
[polaris a11y]: https://polaris.shopify.com/foundations/accessibility#meeting-the-web-content-accessibility-guidelines-wcag
[polaris ds]: https://polaris.shopify.com/design
[polaris com]: https://github.com/Shopify/polaris/discussions
[polaris issue]: https://github.com/Shopify/polaris/issues?q=is%3Aissue+closed%3A%3E2024-01-01+
[polaris con]: https://polaris.shopify.com/contributing

<!-- carbon -->

[carbon]: https://github.com/carbon-design-system/carbon
[carbon fk]: https://carbondesignsystem.com/designing/kits/figma/#external-users
[carbon dv]: https://carbondesignsystem.com/data-visualization/getting-started/
[carbon doc]: https://carbondesignsystem.com/components/overview/components/
[carbon a11y]: https://carbondesignsystem.com/guidelines/accessibility/overview/
[carbon ds]: https://carbondesignsystem.com/designing/get-started/
[carbon com]: https://github.com/carbon-design-system/carbon/discussions
[carbon issue]: https://github.com/carbon-design-system/carbon/issues?q=is%3Aissue+closed%3A%3E2024-01-01+
[carbon con]: https://carbondesignsystem.com/contributing/get-started/

<!-- joy-ui -->

[joy-ui]: https://mui.com/joy-ui/getting-started/
[joy-ui perf]: https://github.com/mui/material-ui/issues/38137
[joy-ui fk]: https://www.figma.com/community/file/1293288155415213351
[joy-ui dv]: https://mui.com/x/react-charts/
[joy-ui doc]: https://mui.com/joy-ui/getting-started/
[joy-ui a11y]: https://mui.com/base-ui/getting-started/accessibility/
[joy-ui com]: https://github.com/mui/material-ui/discussions
[joy-ui issue]: https://github.com/mui/material-ui/issues?q=is%3Aissue+closed%3A%3E2024-01-01+
[joy-ui con]: https://github.com/mui/material-ui/blob/987bb036905fdd4474ae5aaee7b2b5a3dc6b9315/CONTRIBUTING.md

<!-- primer -->

[primer]: https://github.com/primer/react
[primer perf]: https://www.youtube.com/watch?v=OrIuKl_x0vE
[primer fk]: https://www.figma.com/@primer
[primer doc]: https://primer.style/guides/react
[primer a11y]: https://primer.style/guides/accessibility/accessibility-at-github
[primer ds]: https://primer.style/guides/introduction
[primer com]: https://github.com/primer/react/discussions
[primer issue]: https://github.com/primer/react/issues?q=is%3Aissue+closed%3A%3E2024-01-01+
[primer con]: https://primer.style/guides/contribute/how-to-contribute

<!-- atlassian -->

[atlassian]: https://bitbucket.org/atlassian/atlassian-frontend-mirror/src/master/
[atlassian fk]: https://atlassian.design/resources/figma-library
[atlassian doc]: https://atlassian.design/components/
[atlassian a11y]: https://atlassian.design/foundations/accessibility
[atlassian ds]: https://atlassian.design/foundations/
[atlassian con]: https://atlassian.design/resources/contribution

<!-- fluent-ui -->

[fluent-ui]: https://github.com/microsoft/fluentui
[fluent-ui fk]: https://www.figma.com/community/file/836828295772957889
[fluent-ui doc]: https://react.fluentui.dev/?path=/docs/concepts-introduction--page
[fluent-ui a11y]: https://fluent2.microsoft.design/accessibility
[fluent-ui com]: https://github.com/microsoft/fluentui/discussions
[fluent-ui issue]: https://github.com/microsoft/fluentui/issues?q=is%3Aissue+closed%3A%3E2024-01-01+
[fluent-ui ds]: https://fluent2.microsoft.design/design-principles
[fluent-ui con]: https://github.com/microsoft/fluentui/tree/master/docs/react-v9/contributing

<!-- chakra-ui -->

[chakra-ui]: https://github.com/chakra-ui/chakra-ui
[chakra-ui perf]: https://github.com/chakra-ui/chakra-ui/issues/7180
[chakra-ui fk]: https://v2.chakra-ui.com/figma/ui-kit
[chakra-ui doc]: https://v2.chakra-ui.com/docs/components
[chakra-ui com]: https://discord.com/invite/chakra-ui
[chakra-ui ds]: https://v2.chakra-ui.com/getting-started/principles
[chakra-ui issue]: https://github.com/chakra-ui/chakra-ui/issues?q=is%3Aissue+closed%3A%3E2024-01-01+
[chakra-ui con]: https://github.com/chakra-ui/chakra-ui/blob/main/CONTRIBUTING.md

<!-- grommet -->

[grommet]: https://github.com/grommet/grommet
[grommet doc]: https://v2.grommet.io/components
[grommet dv]: https://v2.grommet.io/components#Visualizations
[grommet com]: https://slack-invite.grommet.io/
[grommet issue]: https://github.com/grommet/grommet/issues?q=is%3Aissue+closed%3A%3E2024-01-01+
[grommet con]: https://github.com/grommet/grommet/blob/master/CONTRIBUTING.md

<!-- flowbite -->

[flowbite]: https://github.com/themesberg/flowbite
[flowbite fk]: https://flowbite.com/figma/
[flowbite doc]: https://flowbite.com/docs/getting-started/introduction/
[flowbite dv]: https://flowbite.com/docs/plugins/charts/
[flowbite com]: https://discord.com/invite/S6J9pUmj2t
[flowbite issue]: https://github.com/themesberg/flowbite/issues?q=is%3Aissue+closed%3A%3E2024-01-01+
[flowbite con]: https://github.com/themesberg/flowbite/blob/main/CONTRIBUTING.md

<!-- shadcn-ui -->

[shadcn-ui]: https://github.com/shadcn-ui/ui
[shadcn-ui fk]: https://ui.shadcn.com/docs/figma
[shadcn-ui doc]: https://ui.shadcn.com/docs
[shadcn-ui a11y]: https://www.radix-ui.com/primitives/docs/overview/accessibility
[shadcn-ui dv]: https://ui.shadcn.com/charts
[shadcn-ui com]: https://github.com/shadcn-ui/ui/discussions
[shadcn-ui issue]: https://github.com/shadcn-ui/ui/issues?q=is%3Aissue+closed%3A%3E2024-01-01+
[shadcn-ui con]: https://github.com/shadcn-ui/ui/blob/main/CONTRIBUTING.md

<!-- material-ui -->

[material-ui]: https://mui.com/material-ui/getting-started/
[material-ui perf]: https://github.com/mui/material-ui/issues/38137
[material-ui fk]: https://mui.com/material-ui/design-resources/material-ui-for-figma/
[material-ui dv]: https://mui.com/x/react-charts/
[material-ui doc]: https://mui.com/material-ui/getting-started/
[material-ui a11y]: https://mui.com/base-ui/getting-started/accessibility/
[material-ui com]: https://github.com/mui/material-ui/discussions
[material-ui ds]: https://m3.material.io/
[material-ui issue]: https://github.com/mui/material-ui/issues?q=is%3Aissue+closed%3A%3E2024-01-01+
[material-ui con]: https://github.com/mui/material-ui/blob/987bb036905fdd4474ae5aaee7b2b5a3dc6b9315/CONTRIBUTING.md

<!-- mantine -->

[mantine]: https://mantine.dev/getting-started/
[mantine fk]: https://www.figma.com/community/file/1293978471602433537
[mantine dv]: https://mantine.dev/charts/getting-started/
[mantine doc]: https://mantine.dev/getting-started/
[mantine com]: https://discord.com/invite/wbH82zuWMN
[mantine issue]: https://github.com/mantinedev/mantine/issues?q=is%3Aissue+closed%3A%3E2024-01-01+
[mantine con]: https://mantine.dev/contribute/
