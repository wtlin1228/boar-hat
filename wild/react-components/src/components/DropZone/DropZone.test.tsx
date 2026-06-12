import { expect, test, vi } from 'vitest'
import { render } from 'vitest-browser-react'

import { DropZone } from './DropZone'
import type { DropZoneRenderApi } from './types'

// --- helpers ----------------------------------------------------------------

function makeFile(name: string, size = 10, type = 'text/plain') {
  return new File(['x'.repeat(size)], name, { type })
}

function fileList(files: File[]) {
  const dt = new DataTransfer()
  files.forEach((f) => dt.items.add(f))
  return dt.files
}

// Set the hidden input's files and fire the native change React listens for.
function selectFiles(input: HTMLInputElement, files: File[]) {
  input.files = fileList(files)
  input.dispatchEvent(new Event('change', { bubbles: true }))
}

// Dispatch a drag event carrying files onto the zone.
function fireDrag(
  zone: Element,
  type: 'drop' | 'dragenter' | 'dragleave' | 'dragover',
  files: File[] = [],
) {
  const dataTransfer = new DataTransfer()
  files.forEach((f) => dataTransfer.items.add(f))
  zone.dispatchEvent(new DragEvent(type, { bubbles: true, dataTransfer }))
}

function getInput(container: HTMLElement) {
  return container.querySelector<HTMLInputElement>(
    '[data-testid="rc-dropzone-input"]',
  )!
}

// --- tests ------------------------------------------------------------------

test('renders the label, prompt and browse control', async () => {
  const screen = await render(<DropZone label="Upload here" />)

  await expect.element(screen.getByText('Upload here')).toBeInTheDocument()
  await expect
    .element(screen.getByText('Drag and drop your file here.'))
    .toBeInTheDocument()
  await expect
    .element(screen.getByRole('button', { name: /browse files to upload/i }))
    .toBeInTheDocument()
})

test('browse button opens the file picker via the input', async () => {
  const screen = await render(<DropZone />)
  const clickSpy = vi.spyOn(getInput(screen.container), 'click')

  await screen.getByRole('button', { name: /browse files/i }).click()

  expect(clickSpy).toHaveBeenCalledOnce()
})

test('selecting a file shows it and fires onChange (uncontrolled)', async () => {
  const onChange = vi.fn()
  const screen = await render(<DropZone onChange={onChange} />)

  selectFiles(getInput(screen.container), [makeFile('hello.txt')])

  await expect.element(screen.getByText('hello.txt')).toBeInTheDocument()
  await expect
    .element(screen.getByRole('button', { name: 'Clear all' }))
    .toBeInTheDocument()
  expect(onChange.mock.lastCall?.[0].map((f: File) => f.name)).toEqual([
    'hello.txt',
  ])
})

test('single select keeps only the first file', async () => {
  const screen = await render(<DropZone />)

  selectFiles(getInput(screen.container), [
    makeFile('a.txt'),
    makeFile('b.txt'),
  ])

  await expect.element(screen.getByText('a.txt')).toBeInTheDocument()
  await expect.element(screen.getByText('b.txt')).not.toBeInTheDocument()
})

test('multiple select keeps every file', async () => {
  const screen = await render(<DropZone multiple />)

  selectFiles(getInput(screen.container), [
    makeFile('a.txt'),
    makeFile('b.txt'),
  ])

  await expect.element(screen.getByText('a.txt')).toBeInTheDocument()
  await expect.element(screen.getByText('b.txt')).toBeInTheDocument()
})

test('Clear all empties the selection', async () => {
  const onChange = vi.fn()
  const screen = await render(<DropZone multiple onChange={onChange} />)
  selectFiles(getInput(screen.container), [makeFile('a.txt')])
  await expect.element(screen.getByText('a.txt')).toBeInTheDocument()

  await screen.getByRole('button', { name: 'Clear all' }).click()

  await expect.element(screen.getByText('a.txt')).not.toBeInTheDocument()
  expect(onChange.mock.lastCall?.[0]).toEqual([])
})

test('Remove deletes a single file', async () => {
  const screen = await render(<DropZone multiple />)
  selectFiles(getInput(screen.container), [
    makeFile('a.txt'),
    makeFile('b.txt'),
  ])
  await expect.element(screen.getByText('a.txt')).toBeInTheDocument()

  // Two "Remove" buttons; clicking the first drops a.txt.
  await screen.getByRole('button', { name: 'Remove' }).first().click()

  await expect.element(screen.getByText('a.txt')).not.toBeInTheDocument()
  await expect.element(screen.getByText('b.txt')).toBeInTheDocument()
})

test('getFileError flags invalid files', async () => {
  const screen = await render(
    <DropZone
      multiple
      getFileError={(f) => (f.name === 'bad.txt' ? 'not allowed' : null)}
    />,
  )

  selectFiles(getInput(screen.container), [
    makeFile('ok.txt'),
    makeFile('bad.txt'),
  ])

  await expect.element(screen.getByText('— not allowed')).toBeInTheDocument()
})

test('disabled blocks the input, buttons and drops', async () => {
  const onChange = vi.fn()
  const screen = await render(<DropZone disabled onChange={onChange} />)

  await expect.element(getInput(screen.container)).toBeDisabled()
  await expect
    .element(screen.getByRole('button', { name: /browse files/i }))
    .toBeDisabled()

  fireDrag(screen.getByTestId('rc-dropzone').element(), 'drop', [
    makeFile('a.txt'),
  ])
  expect(onChange).not.toHaveBeenCalled()
})

test('dropping files adds them and fires onChange', async () => {
  const onChange = vi.fn()
  const screen = await render(<DropZone multiple onChange={onChange} />)

  fireDrag(screen.getByTestId('rc-dropzone').element(), 'drop', [
    makeFile('dropped.txt'),
  ])

  await expect.element(screen.getByText('dropped.txt')).toBeInTheDocument()
  expect(onChange.mock.lastCall?.[0].map((f: File) => f.name)).toEqual([
    'dropped.txt',
  ])
})

test('dragenter highlights the zone and dragleave clears it', async () => {
  const screen = await render(<DropZone />)
  const zone = screen.getByTestId('rc-dropzone')

  fireDrag(zone.element(), 'dragenter')
  await expect.element(zone).toHaveClass('rc-dropzone--dragging')

  fireDrag(zone.element(), 'dragleave')
  await expect.element(zone).not.toHaveClass('rc-dropzone--dragging')
})

test('controlled mode emits onChange and renders the value prop', async () => {
  const onChange = vi.fn()
  const screen = await render(
    <DropZone multiple value={[]} onChange={onChange} />,
  )

  // Picking a file does not mutate the displayed list (parent owns it)...
  selectFiles(getInput(screen.container), [makeFile('x.txt')])
  expect(onChange.mock.lastCall?.[0].map((f: File) => f.name)).toEqual([
    'x.txt',
  ])
  await expect.element(screen.getByText('x.txt')).not.toBeInTheDocument()

  // ...until the parent feeds it back through `value`.
  await screen.rerender(
    <DropZone multiple value={[makeFile('x.txt')]} onChange={onChange} />,
  )
  await expect.element(screen.getByText('x.txt')).toBeInTheDocument()
})

test('render prop receives the api and can drive browse()', async () => {
  let captured: DropZoneRenderApi | undefined
  const screen = await render(
    <DropZone multiple>
      {(api) => {
        captured = api
        return (
          <button type="button" onClick={api.browse}>
            Pick
          </button>
        )
      }}
    </DropZone>,
  )
  const clickSpy = vi.spyOn(getInput(screen.container), 'click')

  await screen.getByRole('button', { name: 'Pick' }).click()
  expect(clickSpy).toHaveBeenCalledOnce()

  // The api reflects updated files after a drop.
  fireDrag(screen.getByTestId('rc-dropzone').element(), 'drop', [
    makeFile('r.txt'),
  ])
  await vi.waitFor(() =>
    expect(captured?.files.map((f) => f.name)).toContain('r.txt'),
  )
})
