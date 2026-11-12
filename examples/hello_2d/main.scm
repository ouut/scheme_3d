;; examples/hello_2d/main.scm
;; Run with: cargo run -- examples/hello_2d

(require-builtin macroquad)

(define (init)
  (state-set! 'time 0.0)
  (state-set! 'pulse-radius 36.0))

(define (on-reload)
  (displayln "[hello_2d] Script hot-reloaded!"))

(define (update dt)
  (let ([t (+ (state-get 'time 0.0) dt)])
    (state-set! 'time t)
    (when (is-mouse-button-pressed? 'left)
      (state-set! 'pulse-radius 72.0))
    (let ([r (state-get 'pulse-radius 36.0)])
      (state-set! 'pulse-radius (lerp r 36.0 (clamp (* dt 6.0) 0.0 1.0))))))

(define (draw)
  (clear-background (rgba8 18 22 32 255))

  (let* ([t (state-get 'time 0.0)]
         [w (screen-width)]
         [h (screen-height)]
         [cx (* w 0.5)]
         [cy (* h 0.5)]
         [orbit-x (+ cx (* 140.0 (cos t)))]
         [orbit-y (+ cy (* 90.0 (sin (* t 1.5))))]
         [mouse (mouse-position)]
         [mx (vec2-x mouse)]
         [my (vec2-y mouse)]
         [radius (state-get 'pulse-radius 36.0)])

    ;; Background decorative rings
    (draw-circle-lines cx cy 160.0 2.0 (rgba8 60 80 120 140))
    (draw-poly-lines cx cy 6 110.0 (* t 25.0) 2.5 SKYBLUE)

    ;; Connecting line from center to mouse cursor
    (draw-line cx cy mx my 2.0 (color-with-alpha GOLD 0.6))

    ;; Orbiting satellite
    (draw-circle orbit-x orbit-y 18.0 ORANGE)
    (draw-circle-lines orbit-x orbit-y 24.0 2.0 YELLOW)

    ;; Interactive cursor follower
    (draw-circle mx my radius (color-with-alpha LIME 0.35))
    (draw-circle-lines mx my radius 3.0 GREEN)

    ;; Rotating triangle at center
    (draw-poly cx cy 3 42.0 (* t -45.0) VIOLET)

    ;; HUD overlay
    (draw-rectangle 12 12 430 86 (rgba8 0 0 0 150))
    (draw-rectangle-lines 12 12 430 86 1.5 SKYBLUE)
    (draw-text "scheme_3d - Hello 2D" 24 40 28 WHITE)
    (draw-text "Move mouse, Left-Click to pulse, edit main.scm to hot-reload" 24 66 18 LIGHTGRAY)
    (draw-fps)))
