import React from 'react'
import { gsap, Power2, Power4, Expo } from 'gsap'
import { useGSAP } from '@gsap/react'

import ImagePreloader from '@/image-preloader'

export const useIntroAnimation = ({
  containerRef,
  onAnimationComplete,
}: {
  containerRef: React.RefObject<HTMLDivElement>
  onAnimationComplete: () => void
}) => {
  const [isAllImagesLoaded, setIsAllImagesLoaded] = React.useState(false)

  React.useEffect(() => {
    // Preload all images before starting the animation.
    // We can use querySelectorAll to get all the images, but to keep it simple,
    // we'll hardcode the image paths here for now.
    new ImagePreloader(
      [
        '/assets/img/home/kirby-pink.png',
        '/assets/img/home/kirby-puffy.png',
        '/assets/img/home/kirby-powerful.png',
        '/assets/img/home/kirby-splat-1.png',
        '/assets/img/home/kirby-splat-2.png',
        '/assets/img/intro/kirby-star2.png',
        '/assets/img/intro/metaknight.png',
        '/assets/img/intro/dedede.png',
        '/assets/img/intro/waddledee.png',
        '/assets/img/kirby-logo.png',
      ],
      () => {
        setIsAllImagesLoaded(true)
      }
    )
  }, [])

  // Very Important:
  // This animation is deeply coupled with the HTML structure of the KirbyHeader component.
  useGSAP(
    () => {
      if (!isAllImagesLoaded) {
        return
      }

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
        onComplete: onAnimationComplete,
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
    { scope: containerRef, dependencies: [isAllImagesLoaded] }
  )
}

export default useIntroAnimation
