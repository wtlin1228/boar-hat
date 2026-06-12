import type { ReactNode, Ref } from 'react'

/** Everything a custom renderer needs to rebuild the zone's content. */
export interface DropZoneRenderApi {
  /** Open the native file picker (wire this to your own "browse" button). */
  browse: () => void
  /** The currently selected files. */
  files: File[]
  /** Whether a drag is currently hovering the zone. */
  dragging: boolean
  /** Whether the zone is disabled. */
  disabled: boolean
  /** Whether multiple selection is enabled. */
  multiple: boolean
  /** Remove the file at `index`. */
  remove: (index: number) => void
  /** Empty the whole selection. */
  clear: () => void
  /** Resolve the parent-supplied error for a file, if any. */
  getError: (file: File, index: number) => string | null | undefined
}

export interface DropZoneProps {
  /** Heading shown above the default content. */
  label?: string
  /** Allow selecting/dropping more than one file. Defaults to single. */
  multiple?: boolean
  /** Restrict the picker to certain types, e.g. `.zip,.html` or `image/*`. */
  accept?: string
  /** Disable dropping, browsing and removing. */
  disabled?: boolean

  /** Controlled value. Pass together with `onChange` to own the files. */
  value?: File[]
  /** Initial value for uncontrolled usage (omit `value`). */
  defaultValue?: File[]
  /** Called whenever the selection changes (drop, browse, remove, clear). */
  onChange?: (files: File[]) => void

  /**
   * Flag invalid files. Return an error message for a bad file, or
   * `undefined`/`null` if it's fine. Called for every file. The flagged files
   * are still reported through `onChange` — filtering them out is the caller's
   * responsibility.
   */
  getFileError?: (file: File, index: number) => string | null | undefined

  /**
   * Customize the content. Pass a render function to receive the
   * {@link DropZoneRenderApi} (including `browse()` to open the picker), pass
   * plain nodes, or omit for the built-in content.
   */
  children?: ReactNode | ((api: DropZoneRenderApi) => ReactNode)

  /** Ref to the underlying `<input>` (e.g. to read `.files` directly). */
  ref?: Ref<HTMLInputElement>
}
