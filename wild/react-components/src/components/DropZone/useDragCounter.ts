import { useRef, useState } from 'react'
import type { DragEvent } from 'react'

interface Params {
  disabled: boolean
  onDropFiles: (files: File[]) => void
}

/**
 * Tracks the dragging-over state and returns handlers to spread on the drop
 * target. Uses a depth counter because drag events bubble from children, so a
 * naive `dragleave` would fire (and flicker the highlight) each time the cursor
 * crosses a nested element. We only clear `dragging` when depth returns to 0.
 */
export function useDragCounter({ disabled, onDropFiles }: Params) {
  const [dragging, setDragging] = useState(false)
  const depth = useRef(0)

  const onDrop = (e: DragEvent) => {
    e.preventDefault()
    depth.current = 0
    setDragging(false)
    if (disabled) return
    onDropFiles(Array.from(e.dataTransfer.files))
  }

  // dragover must preventDefault to keep the drop target active, but it fires
  // continuously — don't touch state here, the enter/leave counter owns it.
  const onDragOver = (e: DragEvent) => {
    e.preventDefault()
  }

  const onDragEnter = (e: DragEvent) => {
    e.preventDefault()
    if (disabled) return
    depth.current += 1
    setDragging(true)
  }

  const onDragLeave = (e: DragEvent) => {
    e.preventDefault()
    depth.current -= 1
    if (depth.current <= 0) {
      depth.current = 0
      setDragging(false)
    }
  }

  return {
    dragging,
    dragHandlers: { onDrop, onDragOver, onDragEnter, onDragLeave },
  }
}
