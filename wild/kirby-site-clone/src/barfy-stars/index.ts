import getFPSMeasure from '@/measure-fps'
import Particle from './particle'

const fpsMeasure = getFPSMeasure()

export class BarfyStars {
  private element: HTMLElement

  private running: boolean = false
  private particlesSet: Set<Particle> = new Set()

  // BarfyStars will generate particle elements with the following classes:
  //   .<particleBaseClassName> .<particleBaseClassName>--1
  //   .<particleBaseClassName> .<particleBaseClassName>--2
  //   ...
  //   .<particleBaseClassName> .<particleBaseClassName>--<numUniqueParticles>
  // So, make sure to define the styles for these classes in the CSS.
  // And make sure to provide the correct class name to the BarfyStars instance.
  private numParticles: number
  private numUniqueParticles: number
  private particleBaseClassName: string

  private momentum?: number
  private gravity?: number
  private friction?: number
  private scaleInitial?: number
  private scaleFactor?: number
  private scaleMinimum?: number

  constructor(
    element: HTMLElement,
    {
      numParticles, // ex: 20
      numUniqueParticles, // ex: 3
      particleBaseClassName, // ex: 'BSParticle
      momentum, // Particle has defined the default value
      gravity, // Particle has defined the default value
      friction, // Particle has defined the default value
      scaleInitial, // Particle has defined the default value
      scaleFactor, // Particle has defined the default value
      scaleMinimum, // Particle has defined the default value
    }: {
      numParticles: number
      numUniqueParticles: number
      particleBaseClassName: string
      momentum?: number
      gravity?: number
      friction?: number
      scaleInitial?: number
      scaleFactor?: number
      scaleMinimum?: number
    }
  ) {
    this.element = element

    this.numParticles = numParticles
    this.numUniqueParticles = numUniqueParticles
    this.particleBaseClassName = particleBaseClassName

    this.momentum = momentum
    this.gravity = gravity
    this.friction = friction
    this.scaleInitial = scaleInitial
    this.scaleFactor = scaleFactor
    this.scaleMinimum = scaleMinimum
  }

  public barf = () => {
    this.addParticles()
  }

  private run = () => {
    this.particlesSet.forEach((particle) => {
      particle.run()
    })

    if (this.running) requestAnimationFrame(this.run)
  }

  private addParticles = () => {
    for (let i = 0; i < this.numParticles; i++) {
      this.addParticle()
    }
    this.running = true
    this.run()
  }

  private addParticle = () => {
    if (fpsMeasure.averageFpsInLast60Frame > 20) {
      const particle = new Particle({
        momentumFactor: this.momentum,
        friction: this.friction,
        scaleInitial: this.scaleInitial,
        scaleFactor: this.scaleFactor,
        scaleMinimum: this.scaleMinimum,
        gravityFactor: this.gravity,
        particleClasses: this.getParticleClasses(),
        removeParticle: this.removeParticle,
      })
      if (particle.element) {
        this.particlesSet.add(particle)
        this.element.appendChild(particle.element)
      }
    }
  }

  private getParticleClasses = (): string => {
    const c = this.particleBaseClassName
    const randomNumber = Math.ceil(Math.random() * this.numUniqueParticles)
    return `${c} ${c}--${randomNumber}`
  }

  private removeParticle = (particle: Particle) => {
    if (this.particlesSet.has(particle)) {
      this.element.removeChild(particle.element)
      this.particlesSet.delete(particle)
      if (this.particlesSet.size === 0) {
        this.running = false
      }
    }
  }
}

export default BarfyStars
