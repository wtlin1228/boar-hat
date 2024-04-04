export class ImagePreloader {
  private numberOfImagesToPreload: number
  private loaded: number
  private onLoaded: () => void

  constructor(images: string[], onLoaded: () => void) {
    this.numberOfImagesToPreload = images.length
    this.loaded = 0
    this.onLoaded = onLoaded

    images.forEach(this.preload)
  }

  incrementLoaded = () => {
    this.loaded += 1
    if (this.loaded == this.numberOfImagesToPreload) {
      this.onLoaded()
    }
  }

  preload = (imageSrc: string) => {
    const image = new Image()
    image.onload = this.incrementLoaded
    image.src = imageSrc
  }
}

export default ImagePreloader
