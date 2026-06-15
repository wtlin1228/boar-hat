import { useCallback, useRef } from 'react'

import './DropZone.css'
import type { DropZoneProps, DropZoneRenderApi, DropZoneState } from './types'
import { useFileSelection } from './useFileSelection'
import { useDragCounter } from './useDragCounter'
import { DefaultContent, RenderContent } from './DropZoneContent'

/**
 * A drag-and-drop file zone supporting single/multiple selection, controlled
 * and uncontrolled usage, type restrictions, per-file error flagging, and
 * fully customizable content via a render prop.
 */
export function DropZone({
  label,
  multiple = false,
  accept,
  disabled = false,
  className,
  style,
  value,
  defaultValue,
  onChange,
  getFileError,
  children,
  ref,
}: DropZoneProps) {
  const inputRef = useRef<HTMLInputElement>(null)

  const { files, addFiles, remove, clear } = useFileSelection({
    multiple,
    value,
    defaultValue,
    onChange,
    inputRef,
  })

  const { dragging, dragHandlers } = useDragCounter({
    disabled,
    onDropFiles: addFiles,
  })

  const browse = useCallback(() => inputRef.current?.click(), [])

  // Merge our two refs (internal + forwarded) onto the same input node.
  // useCallback keeps the identity stable — an inline callback ref would be
  // detached/reattached (called with null then the node) on every render,
  // which would also reset the consumer's forwarded ref each time.
  const setRef = useCallback(
    (node: HTMLInputElement | null) => {
      inputRef.current = node
      if (typeof ref === 'function') ref(node)
      else if (ref) ref.current = node
    },
    [ref],
  )

  const api: DropZoneRenderApi = {
    browse,
    files,
    dragging,
    disabled,
    multiple,
    remove,
    clear,
    getError: (file, i) => getFileError?.(file, i),
  }

  // Render-prop > plain nodes > built-in default content. The render prop is
  // invoked through RenderContent (a component boundary) rather than called
  // inline, so the ref-backed `api` is passed as a prop.
  const content =
    typeof children === 'function' ? (
      <RenderContent render={children} api={api} />
    ) : (
      (children ?? <DefaultContent api={api} label={label} />)
    )

  const state: DropZoneState = { dragging, disabled }
  const resolvedClassName =
    typeof className === 'function' ? className(state) : className
  const resolvedStyle = typeof style === 'function' ? style(state) : style

  const containerClassName = [
    'rc-dropzone',
    dragging && 'rc-dropzone--dragging',
    disabled && 'rc-dropzone--disabled',
    resolvedClassName,
  ]
    .filter(Boolean)
    .join(' ')

  return (
    <div
      {...dragHandlers}
      className={containerClassName}
      style={resolvedStyle}
      data-testid="rc-dropzone"
    >
      {content}

      {/* Hidden trigger backing api.browse(). */}
      <input
        ref={setRef}
        type="file"
        multiple={multiple}
        accept={accept}
        disabled={disabled}
        hidden
        data-testid="rc-dropzone-input"
        onChange={(e) => addFiles(Array.from(e.target.files ?? []))}
      />
    </div>
  )
}
