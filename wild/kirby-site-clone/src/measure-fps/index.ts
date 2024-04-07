/**
 * A singleton class that provides Framerate information for a website. When running, this will produce a
 * number of useful internal properties.
 *
 * - currentFps
 *   The current framerate
 * - lowestFps
 *   The lowest overall framerate
 * - averageFpsInLast60Frame
 *   The average framerate in the last 60 frames (ideally this is a second)
 * - averageFpsOverall
 *   The average overall framerate
 *
 * ## Usage
 * ```
 * const fps = utilities.getFPSMeasure();
 * console.log(fps.currentFps); // 60
 * ```
 *
 * When using this class, it is often fortuitous to cycle it down and back up after a big FPS dip:
 * ```
 * fps.stop();
 * fps.start();
 * ```
 *
 */

const initLastTimeStamp = null
const initCurrentFps = 0
const initLowestFps = 60
const initAverageFpsInLast60Frame = 60
const initAverageFpsOverall = 60
const initFrameCount = 0

class MeasureFPS {
  private running: boolean = false
  private lastTimestamp: DOMHighResTimeStamp | null = initLastTimeStamp

  // used for calculating average fps in the last 60 frames
  private fpsRecord: number[] = Array(60).fill(60)
  private fpsRecordIndex: number = 0
  private fpsRecordSum: number = 60 * 60

  // used for calculating average fps overall
  private frameCount: number = initFrameCount

  public currentFps: number = initCurrentFps
  public lowestFps: number = initLowestFps
  public averageFpsInLast60Frame: number = initAverageFpsInLast60Frame
  public averageFpsOverall: number = initAverageFpsOverall

  constructor() {
    this.start()
  }

  public stop = () => {
    this.running = false
  }

  public start = () => {
    if (this.running === true) {
      return
    }
    this.running = true
    this.lastTimestamp = initLastTimeStamp

    this.fpsRecord = Array(60).fill(60)
    this.fpsRecordIndex = 0
    this.fpsRecordSum = 60 * 60

    this.frameCount = initFrameCount

    this.currentFps = initCurrentFps
    this.lowestFps = initLowestFps
    this.averageFpsInLast60Frame = initAverageFpsInLast60Frame
    this.averageFpsOverall = initAverageFpsOverall

    requestAnimationFrame(this.run)
  }

  private run = (timeStamp: DOMHighResTimeStamp) => {
    if (this.running === false) {
      return
    } else if (this.lastTimestamp == null) {
      // skip the first frame
      this.lastTimestamp = timeStamp
      requestAnimationFrame(this.run)
      return
    } else if (this.lastTimestamp >= timeStamp) {
      // skip the wired frame
      requestAnimationFrame(this.run)
      return
    }

    const elapsedTime = timeStamp - this.lastTimestamp
    this.lastTimestamp = timeStamp
    this.frameCount += 1

    this.currentFps = 1000 / elapsedTime
    if (this.currentFps < this.lowestFps) {
      this.lowestFps = this.currentFps
    }

    this.averageFpsOverall =
      this.averageFpsOverall +
      (this.currentFps - this.averageFpsOverall) / this.frameCount

    this.fpsRecordSum -= this.fpsRecord[this.fpsRecordIndex]
    this.fpsRecordSum += this.currentFps
    this.fpsRecord[this.fpsRecordIndex] = this.currentFps
    this.fpsRecordIndex = (this.fpsRecordIndex + 1) % 60
    this.averageFpsInLast60Frame = this.fpsRecordSum / 60

    requestAnimationFrame(this.run)
  }
}

let singleton: MeasureFPS | null = null

const getFPSMeasure = () => {
  if (singleton === null) {
    singleton = new MeasureFPS()
  }
  return singleton
}

export default getFPSMeasure
