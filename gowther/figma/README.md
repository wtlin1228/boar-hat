# Figma Features

- Components and instances
  - Detached instances can't be linked to the original component again
  - Some properties of an instance can be modified without detaching, like corner radius
- Auto layout is like CSS's flexbox

## Section vs Frame vs Group

- Section is for organization
- Frame is a container for our content
- Group is a collection of layers

## Hug vs Fill

- Hug: content determines the width, container hugs the content
- Fill: parent determines the width, content fills the container

# Considerations

## How to do version control for the component library?

My suggestion: Avoid relying on Figma’s branching feature, as changes in branches can’t be published unless they’re merged into master. Instead, we can duplicate the current component file (e.g., version `1.2.0`) and create a new versioned copy, such as `1.3.0`, `1.2.1`, or `2.0.0`, depending on the scope of the changes. This approach allows each version to be published independently and ensures older versions remain available for apps that haven’t yet upgraded.

## How to migrate design files to use newer version of the component library

My suggestion: There’s currently no automated way to migrate a design file from version `1.2.0` to `1.2.1` of the component library. The migration must be done manually, or we can explore using community plugins, or even create our own, to help streamline the process.

# Reference

- https://www.figma.com/developers
- https://help.figma.com/hc/en-us/sections/30880632542743-Figma-Design-for-beginners
- https://help.figma.com/hc/en-us/sections/6448765398551-Build-your-first-plugin
