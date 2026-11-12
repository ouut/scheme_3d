;; examples/basic_3d/main.scm
;; Multi-file 3D example with hot-reload & persistent camera state
;; Run with: cargo run -- examples/basic_3d

(require-builtin macroquad)
(require "scheme3d/prelude")
(require "camera.scm")
(require "scene.scm")

(define (init)
  (ensure-camera-state!)
  (state-set! 'sim-time 0.0)
  (state-set! 'paused #f))

(define (on-reload)
  (ensure-camera-state!)
  (displayln "[basic_3d] Scene & camera modules hot-reloaded!"))

(define (update dt)
  (when (is-key-pressed? 'space)
    (state-set! 'paused (not (state-get 'paused #f))))
  (update-orbit-camera! dt)
  (unless (state-get 'paused #f)
    (state-set! 'sim-time (+ (state-get 'sim-time 0.0) dt))))

(define (draw)
  (clear-background (rgba8 14 17 24 255))

  (let ([cam (build-active-camera)]
        [t (state-get 'sim-time 0.0)])
    (with-camera-3d cam
      (lambda ()
        (draw-3d-scene t))))

  ;; 2D HUD overlay (automatically restored after with-camera-3d)
  (draw-rectangle 12 12 500 112 (rgba8 0 0 0 165))
  (draw-rectangle-lines 12 12 500 112 1.5 GOLD)
  (draw-text "scheme_3d - Multi-Module 3D Demo" 24 38 26 WHITE)
  (draw-text "Controls: WASD / Arrows = Orbit | Q/E/Wheel = Zoom | Space = Pause | R = Reset" 24 64 17 LIGHTGRAY)
  (draw-text (camera-status-text) 24 90 18 SKYBLUE)
  (draw-text "Edit examples/basic_3d/scene.scm and save to see instant hot-reload!" 24 112 16 GOLD)
  (draw-fps))
