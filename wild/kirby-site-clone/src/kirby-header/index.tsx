import React from 'react'
import useIntroAnimation from './useIntroAnimation'
import BarfyStars from '@/barfy-stars'

function KirbyHeader() {
  const [isIntroFinished, setIsIntroFinished] = React.useState(false)

  const containerRef = React.useRef<HTMLDivElement>(null)
  const learnMoreButtonRef = React.useRef<HTMLDivElement>(null)

  useIntroAnimation({
    containerRef,
    onAnimationComplete: () => {
      setIsIntroFinished(true)
    },
  })

  React.useEffect(() => {
    if (learnMoreButtonRef.current) {
      const learnMoreButton = learnMoreButtonRef.current

      const barfStars = new BarfyStars(learnMoreButtonRef.current, {
        numParticles: 24,
        // We have 4 variants of the star particles for now
        // ref: src/index.css@.BSParticle--
        numUniqueParticles: 4,
        // have to be 'BSParticle' since we're providing the styles through the CSS
        particleBaseClassName: 'BSParticle',
      })

      learnMoreButton.addEventListener('pointerenter', barfStars.barf)
      return () => {
        learnMoreButton.removeEventListener('pointerenter', barfStars.barf)
      }
    }
  }, [])

  return (
    <div
      className="hero"
      ref={containerRef}
    >
      {!isIntroFinished && (
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
      )}
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
                className="starburst cta stars"
                ref={learnMoreButtonRef}
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
