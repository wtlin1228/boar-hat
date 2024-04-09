import getFPSMeasure from '@/measure-fps'

const fpsMeasure = getFPSMeasure()

export class Particle {
  private friction: number
  private scaleMinimum: number
  private gravityFactor: number

  private momentumX: number
  private momentumY: number
  private positionX: number
  private positionY: number
  private scale: number
  private opacity: number
  private rotation: number

  public element: HTMLSpanElement
  private removeParticle: (particle: Particle) => void

  constructor({
    momentumFactor = 5.0,
    friction = 0.999,
    scaleInitial = 0.5,
    scaleFactor = 0.8,
    scaleMinimum = 0.05,
    gravityFactor = 0.4,
    particleClasses,
    removeParticle,
  }: {
    momentumFactor?: number
    friction?: number
    scaleInitial?: number
    scaleFactor?: number
    scaleMinimum?: number
    gravityFactor?: number
    particleClasses: string
    removeParticle: (particle: Particle) => void
  }) {
    this.friction = friction
    this.scaleMinimum = scaleMinimum
    this.gravityFactor = gravityFactor

    this.element = document.createElement('span')
    this.element.className = particleClasses
    this.removeParticle = removeParticle

    this.momentumX = (Math.random() * 2 - 1) * momentumFactor
    this.momentumY =
      (1 + Math.random() * (1 + gravityFactor)) * momentumFactor * -1
    this.positionX = 0
    this.positionY = 0

    this.scale = scaleInitial + Math.random() * scaleFactor
    this.opacity = 1
    this.rotation = this.momentumX

    this.run()
  }

  public run = () => {
    this.momentumX *= this.friction
    this.momentumY *= this.friction
    this.momentumY += this.gravityFactor
    this.positionX += this.momentumX
    this.positionY += this.momentumY
    this.rotation += this.momentumX
    this.scale *= this.friction - 0.04
    this.opacity *= this.friction + 0.01

    this.element.style.transform = `translate(${this.positionX}px, ${this.positionY}px) scale(${this.scale}) rotate(${this.rotation}deg)`
    this.element.style.opacity = this.opacity.toString()

    if (
      this.scale < this.scaleMinimum ||
      fpsMeasure.averageFpsInLast60Frame < 10
    ) {
      this.removeParticle(this)
    }
  }
}

export default Particle
