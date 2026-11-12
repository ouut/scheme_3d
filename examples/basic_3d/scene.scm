;; examples/basic_3d/scene.scm
;; 3D scene rendering module

(require-builtin macroquad)

(provide draw-3d-scene)

(define (draw-3d-scene t)
  ;; Reference ground grid & axes
  (draw-grid 24 1.0 DARKGRAY (rgba8 50 58 75 255))
  (draw-line-3d (vec3 0 0.02 0) (vec3 3 0.02 0) RED)
  (draw-line-3d (vec3 0 0.02 0) (vec3 0 3.0 0) GREEN)
  (draw-line-3d (vec3 0 0.02 0) (vec3 0 0.02 3) BLUE)

  ;; Central pulsing cube with wireframe
  (let* ([bob (+ 1.25 (* 0.35 (sin (* t 2.2))))]
         [center (vec3 0.0 bob 0.0)]
         [size (vec3 1.6 1.6 1.6)])
    (draw-cube center size SKYBLUE)
    (draw-cube-wires center size WHITE))

  ;; Orbiting sphere
  (let* ([angle (* t 1.4)]
         [sx (* 3.6 (cos angle))]
         [sz (* 3.6 (sin angle))]
         [spos (vec3 sx 1.0 sz)])
    (draw-sphere spos 0.75 ORANGE)
    (draw-sphere-wires spos 0.78 GOLD)
    (draw-line-3d (vec3 0.0 1.25 0.0) spos YELLOW))

  ;; Pillars / cones around the arena
  (draw-cylinder (vec3 -3.5 0.0 -3.5) 0.45 0.45 2.4 16 PURPLE)
  (draw-cylinder-wires (vec3 -3.5 0.0 -3.5) 0.46 0.46 2.4 16 VIOLET)

  (draw-cylinder (vec3 3.5 0.0 -3.5) 0.0 0.7 2.2 16 LIME)
  (draw-cylinder-wires (vec3 3.5 0.0 -3.5) 0.0 0.71 2.2 16 GREEN))
