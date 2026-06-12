import type { ReactNode } from 'react'
import type { DropZoneRenderApi } from './types'

/**
 * Invokes a render prop behind a component boundary so the ref-backed `api`
 * arrives via props rather than being read in the parent's render scope —
 * which satisfies the "no refs during render" lint rule.
 */
export function RenderContent({
  render,
  api,
}: {
  render: (api: DropZoneRenderApi) => ReactNode
  api: DropZoneRenderApi
}) {
  return <>{render(api)}</>
}

/** The built-in look — also a reference for what a custom renderer can build. */
export function DefaultContent({
  api,
  label,
}: {
  api: DropZoneRenderApi
  label?: string
}) {
  const { browse, files, disabled, multiple, remove, clear, getError } = api
  return (
    <>
      {label && <p className="rc-dropzone__heading">{label}</p>}
      <p className="rc-dropzone__subtext">
        Drag and drop your {multiple ? 'files' : 'file'} here.
      </p>
      <p className="rc-dropzone__hint">
        Or{' '}
        <button
          type="button"
          className="rc-dropzone__link"
          onClick={browse}
          disabled={disabled}
        >
          browse files to upload
        </button>
        .
      </p>

      {files.length > 0 && (
        <div className="rc-dropzone__files">
          <strong>{files.length} file(s) selected</strong>
          <ul className="rc-dropzone__file-list">
            {files.map((file, i) => {
              const error = getError(file, i)
              return (
                <li key={`${file.name}-${file.lastModified}-${i}`}>
                  <span
                    className={error ? 'rc-dropzone__name--error' : undefined}
                  >
                    {file.name}
                  </span>{' '}
                  <span className="rc-dropzone__size">({file.size} bytes)</span>
                  {error && (
                    <span className="rc-dropzone__error"> — {error}</span>
                  )}
                  <button
                    type="button"
                    className="rc-dropzone__remove"
                    onClick={() => remove(i)}
                    disabled={disabled}
                  >
                    Remove
                  </button>
                </li>
              )
            })}
          </ul>
          <button type="button" onClick={clear} disabled={disabled}>
            Clear all
          </button>
        </div>
      )}
    </>
  )
}
