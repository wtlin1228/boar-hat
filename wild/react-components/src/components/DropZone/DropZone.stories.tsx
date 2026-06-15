import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { DropZone } from './DropZone'

const MAX_SIZE = 1024 * 1024 // 1 MB

const meta = {
  title: 'Components/DropZone',
  component: DropZone,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
  argTypes: {
    onChange: { action: 'changed' },
  },
  args: {
    label: 'Drag and drop your files here',
  },
} satisfies Meta<typeof DropZone>

export default meta
type Story = StoryObj<typeof meta>

/** Single-file, uncontrolled — the component manages its own selection. */
export const Single: Story = {
  args: {
    label: 'Drop a file (single)',
  },
}

/** Multiple selection keeps every dropped/picked file. */
export const Multiple: Story = {
  args: {
    label: 'Drop files (multiple)',
    multiple: true,
  },
}

/** `accept` restricts the native picker to the given types. */
export const RestrictedTypes: Story = {
  args: {
    label: 'Images only',
    multiple: true,
    accept: 'image/*',
  },
}

/**
 * Controlled — the parent owns the file list via `value` + `onChange`. The
 * count below is rendered from the parent's state to prove it drives the UI.
 */
export const Controlled: Story = {
  args: { label: 'Controlled by parent state', multiple: true },
  render: (args) => {
    const [files, setFiles] = useState<File[]>([])
    return (
      <div>
        <DropZone {...args} value={files} onChange={setFiles} />
        <p>
          Parent holds <strong>{files.length}</strong> file(s):{' '}
          {files.map((f) => f.name).join(', ') || '(none)'}
        </p>
      </div>
    )
  },
}

/** Disabled — dropping, browsing and removing are all blocked. */
export const Disabled: Story = {
  args: {
    label: 'Disabled',
    disabled: true,
  },
}

/** `getFileError` flags files over 1 MB; they render in red but still emit. */
export const WithError: Story = {
  args: {
    label: 'Files over 1 MB are flagged',
    multiple: true,
    getFileError: (file) =>
      file.size > MAX_SIZE ? 'too large (max 1 MB)' : null,
  },
}

/**
 * Customize the container per state. `style` (and `className`) accept a
 * function of the current {@link DropZoneState}, so the *normal*, *dragging*
 * and *disabled* states can each be styled respectively — here the border and
 * background change on drag.
 */
export const CustomContainer: Story = {
  args: {
    label: 'Drag a file to see the container restyle',
    style: ({ dragging, disabled }) => ({
      borderStyle: 'solid',
      borderColor: dragging ? '#d21419' : disabled ? '#646d72' : '#e76125',
      background: dragging ? 'rgba(210, 20, 25, 0.08)' : '#ececec',
      color: '#646d72',
    }),
  },
}

/** Fully custom content via the render prop, using `browse()` and `files`. */
export const CustomContent: Story = {
  args: { multiple: true },
  render: (args) => (
    <DropZone {...args}>
      {({ browse, files, dragging, clear }) => (
        <div>
          <p className="rc-dropzone__heading">
            {dragging
              ? 'Release to drop! 🎯'
              : `${files.length} file(s) ready 📦`}
          </p>
          <button type="button" onClick={browse}>
            Pick files
          </button>
          {files.length > 0 && (
            <button
              type="button"
              onClick={clear}
              className="rc-dropzone__remove"
            >
              Reset
            </button>
          )}
        </div>
      )}
    </DropZone>
  ),
}
