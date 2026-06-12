import { useCallback, useState } from 'react'
import type { RefObject } from 'react'

interface Params {
  multiple: boolean
  value?: File[]
  defaultValue?: File[]
  onChange?: (files: File[]) => void
  inputRef: RefObject<HTMLInputElement | null>
}

/**
 * Owns the selected files for both controlled and uncontrolled modes and keeps
 * the native `<input>`'s FileList in sync, so reading `.files` off the ref
 * always matches what's rendered.
 */
export function useFileSelection({
  multiple,
  value,
  defaultValue,
  onChange,
  inputRef,
}: Params) {
  // Controlled when `value` is supplied; otherwise we track files internally.
  const isControlled = value !== undefined
  const [internalFiles, setInternalFiles] = useState<File[]>(defaultValue ?? [])
  const files = isControlled ? value : internalFiles

  const commit = useCallback(
    (next: File[]) => {
      // Single-select keeps only the first file even if more come in.
      const trimmed = multiple ? next : next.slice(0, 1)
      syncInputFiles(inputRef.current, trimmed)
      if (!isControlled) setInternalFiles(trimmed)
      onChange?.(trimmed)
    },
    [multiple, isControlled, onChange, inputRef],
  )

  const addFiles = useCallback(
    (incoming: File[]) => {
      if (incoming.length > 0) commit(incoming)
    },
    [commit],
  )

  const remove = useCallback(
    (index: number) => commit(files.filter((_, j) => j !== index)),
    [commit, files],
  )

  const clear = useCallback(() => commit([]), [commit])

  return { files, addFiles, remove, clear }
}

/**
 * A FileList can't be constructed directly, but assigning
 * `input.files = dataTransfer.files` is allowed — that's how we mirror our
 * state back onto the input (e.g. so removals also empty the input).
 */
function syncInputFiles(input: HTMLInputElement | null, files: File[]) {
  if (!input) return
  const dt = new DataTransfer()
  files.forEach((file) => dt.items.add(file))
  input.files = dt.files
}
