import React from 'react'
import { gsap, Power2, Power4, Expo } from 'gsap'
import { useGSAP } from '@gsap/react'

function KirbyHeader() {
  const [isIntroFinished, setIsIntroFinished] = React.useState(false)

  const containerRef = React.useRef<HTMLDivElement>(null)

  // Intro Animation
  useGSAP(
    () => {
      const frames = [
        {
          frame: '.hero__frame--1',
          bg: '.hero__bg--pink',
          imgTweenVars: {
            xPercent: 0,
            yPercent: 110,
            scale: 1,
          },
          txtTweenVars: {
            x: 120,
            opacity: 0,
          },
        },
        {
          frame: '.hero__frame--2',
          bg: '.hero__bg--puffy',
          imgTweenVars: {
            xPercent: -110,
            yPercent: 110,
            scale: 0,
          },
          txtTweenVars: {
            x: -100,
            opacity: 0,
          },
        },
        {
          frame: '.hero__frame--3',
          bg: '.hero__bg--powerful',
          imgTweenVars: {
            xPercent: 250,
            yPercent: 0,
            scale: 1,
          },
          txtTweenVars: {
            x: 100,
            opacity: 0,
          },
        },
      ]

      const tl = gsap.timeline({
        paused: true,
        onComplete: () => {
          setIsIntroFinished(true)
        },
      })

      // 1. Pink Kirby shows up
      // 2. Puffy Kirby shows up
      // 3. Powerful Kirby shows up
      frames.forEach(({ frame, bg, imgTweenVars, txtTweenVars }, index) => {
        tl.addLabel('frame-start-' + index)
        tl.to(frame, { opacity: 1, duration: 0.2 }, 'frame-start-' + index)
        tl.from(
          `${frame} .hero__frame__img`,
          { ...imgTweenVars, duration: 1.2, ease: Power4.easeInOut },
          '-=.1'
        )
        tl.from(
          `${frame} .hero__frame__txt`,
          {
            ...txtTweenVars,
            duration: 0.6,
            ease: Expo.easeOut,
          },
          '-=.5'
        )
        tl.addLabel('frame-end-' + index, '+=.6')
        tl.to([frame, bg], { opacity: 0, duration: 0.4 }, 'frame-end-' + index)
      })

      // 4. Kirby comes in
      tl.set('.hero-splat', { display: 'block' })
      tl.to('.hero-splat .hero-splat__img--incoming', {
        scale: 1.2,
        opacity: 1,
        duration: 1,
        ease: Power2.easeIn,
      })
      tl.set('.hero-splat .hero-splat__img--incoming', {
        opacity: 0,
      })

      // 5. Kirby squishes
      tl.set('.hero-splat .hero-splat__img--squished', {
        opacity: 1,
      })
      tl.set('.hero__final', { opacity: 1 })
      tl.addLabel('splat', '+=0.35')
      tl.to(
        '.hero-splat .hero-splat__img--squished',

        { yPercent: 100, opacity: 0, duration: 2.7, ease: Power2.easeOut },
        'splat'
      )

      // 6. Kirby and friends show up
      tl.from(
        '.hero__image-group img',
        {
          stagger: 0.2,
          left: '0%',
          top: '-20%',
          scale: 0,
          opacity: 0,
          rotation: -20 + Math.random() * 40,
          clearProps: 'left,top,rotation',
          ease: Power4.easeInOut,
          duration: 0.5,
        },
        'splat'
      )
      tl.from(
        'h1 span.img',
        {
          left: '70%',
          scale: 0,
          opacity: 0,
          rotation: 360 + Math.random() * 40,
          duration: 0.6,
          ease: Power4.easeIn,
        },
        '-=1.5'
      )

      tl.from(
        '.hero__content .transition',
        { opacity: 0, duration: 0.5, stagger: 0.2 },
        '-=1.4'
      )
      tl.call(() => {
        tl.set('.hero-splat', { display: 'none' })
      })

      tl.play()
    },
    { scope: containerRef }
  )

  return (
    <div
      className="hero"
      ref={containerRef}
    >
      <div className="hero-splat">
        <div className="hero-splat__img hero-splat__img--incoming">
          <img
            src="/assets/img/home/kirby-splat-1.png"
            alt=""
            width="717"
          />
        </div>
        <div className="hero-splat__img hero-splat__img--squished">
          <img
            src="/assets/img/home/kirby-splat-2.png"
            alt=""
            width="1139"
          />
        </div>
      </div>
      <div className="hero-wrapper">
        <div className="hero__bg__container">
          <div className="hero__bg hero__bg--final" />
          {!isIntroFinished && (
            <>
              <div className="hero__bg hero__bg--powerful" />
              <div className="hero__bg hero__bg--puffy" />
              <div className="hero__bg hero__bg--pink" />
            </>
          )}
        </div>

        <div className="hero__spiral">
          <div className="hero__spiral__pattern"></div>
        </div>

        {!isIntroFinished && (
          <>
            {/* 1: Pink Kirby */}
            <div className="hero__frame hero__frame--1">
              <div className="hero__frame__img">
                <img
                  className="img-max"
                  src="/assets/img/home/kirby-pink.png"
                  alt="Pink Kirby"
                />
              </div>
              <div className="hero__frame__txt">
                <span className="h1 title-1">Pink</span>
              </div>
            </div>

            {/* 2: Puffy Kirby */}
            <div className="hero__frame hero__frame--2">
              <div className="hero__frame__img">
                <img
                  className="img-max"
                  src="/assets/img/home/kirby-puffy.png"
                  alt="Puffy Kirby"
                />
              </div>
              <div className="hero__frame__txt">
                <span className="h1 title-1">Puffy</span>
              </div>
            </div>

            {/* 3: Powerful Kirby */}
            <div className="hero__frame hero__frame--3">
              <div className="hero__frame__img">
                <img
                  className="img-max"
                  src="/assets/img/home/kirby-powerful.png"
                  alt="Powerful Kirby"
                />
              </div>
              <div className="hero__frame__txt">
                <span className="h1 title-1">Powerful!</span>
              </div>
            </div>
          </>
        )}

        {/* hero final */}
        <div className="hero__final">
          <div className="row-flex">
            <div className="column-flex-12 column-flex-large-5 hero__content">
              <h1 className="title-1">
                <span className="transition">
                  Official home of
                  <span className="visually-hidden">Kirby</span>
                </span>
                <span className="img">
                  <img
                    src="/assets/img/kirby-logo.png"
                    alt=""
                  />
                </span>
              </h1>
              <p className="transition">
                Don’t let the adorable face fool you—this powerful, pink puff
                can pack a punch! Since 1992, Kirby has been battling baddies
                across dozens of games. With his unique abilities, Kirby is
                always finding new ways to take on troublemakers.
              </p>
              <div
                className="cta stars"
                data-controller="BarfyStars"
              >
                <a
                  className="button button--blue"
                  href="/about/"
                >
                  <span>Learn more</span>
                </a>
              </div>
            </div>
            <div className="column-flex-12 column-flex-large-7 hero__image-group">
              <img
                className="kirby"
                src="/assets/img/intro/kirby-star2.png"
                alt=""
              />
              <img
                className="metaknight"
                src="/assets/img/intro/metaknight.png"
                alt=""
              />
              <img
                className="dedede"
                src="/assets/img/intro/dedede.png"
                alt=""
              />
              <img
                className="waddledee"
                src="/assets/img/intro/waddledee.png"
                alt=""
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default KirbyHeader
