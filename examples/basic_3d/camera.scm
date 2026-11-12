;; examples/basic_3d/camera.scm
;; Orbit + zoom 3D camera controller module

(require-builtin macroquad)

(provide ensure-camera-state!
         update-orbit-camera!
         build-active-camera
         camera-status-text)

(define (ensure-camera-state!)
  (unless (state-has? 'cam-yaw)
    (state-set! 'cam-yaw 0.65))
  (unless (state-has? 'cam-pitch)
    (state-set! 'cam-pitch 0.42))
  (unless (state-has? 'cam-dist)
    (state-set! 'cam-dist 11.5))
  (unless (state-has? 'cam-target)
    (state-set! 'cam-target (vec3 0.0 1.0 0.0))))

(define (update-orbit-camera! dt)
  (ensure-camera-state!)
  (let* ([turn-speed 1.8]
         [zoom-speed 8.0]
         [yaw0 (state-get 'cam-yaw 0.65)]
         [pitch0 (state-get 'cam-pitch 0.42)]
         [dist0 (state-get 'cam-dist 11.5)]
         [wheel-y (vec2-y (mouse-wheel))]
         [dyaw (+ (if (or (is-key-down? 'left) (is-key-down? 'a)) (- turn-speed) 0.0)
                  (if (or (is-key-down? 'right) (is-key-down? 'd)) turn-speed 0.0))]
         [dpitch (+ (if (or (is-key-down? 'up) (is-key-down? 'w)) turn-speed 0.0)
                    (if (or (is-key-down? 'down) (is-key-down? 's)) (- turn-speed) 0.0))]
         [ddist (+ (if (is-key-down? 'q) (- zoom-speed) 0.0)
                   (if (is-key-down? 'e) zoom-speed 0.0))]
         [yaw (+ yaw0 (* dyaw dt))]
         [pitch (clamp (+ pitch0 (* dpitch dt)) -1.2 1.35)]
         [dist (clamp (- (+ dist0 (* ddist dt)) (* wheel-y 0.8)) 3.5 28.0)])
    (when (is-key-pressed? 'r)
      (state-set! 'cam-yaw 0.65)
      (state-set! 'cam-pitch 0.42)
      (state-set! 'cam-dist 11.5))
    (unless (is-key-pressed? 'r)
      (state-set! 'cam-yaw yaw)
      (state-set! 'cam-pitch pitch)
      (state-set! 'cam-dist dist))))

(define (build-active-camera)
  (ensure-camera-state!)
  (let* ([yaw (state-get 'cam-yaw 0.65)]
         [pitch (state-get 'cam-pitch 0.42)]
         [dist (state-get 'cam-dist 11.5)]
         [target (state-get 'cam-target (vec3 0.0 1.0 0.0))]
         [horiz (* dist (cos pitch))]
         [eye (vec3+ target
                     (vec3 (* horiz (cos yaw))
                           (* dist (sin pitch))
                           (* horiz (sin yaw))))])
    (make-camera-3d eye target (vec3 0.0 1.0 0.0) (deg->rad 45.0))))

(define (camera-status-text)
  (let ([pos (camera-3d-position (build-active-camera))])
    (string-append
     "Cam Pos: ("
     (number->string (round (vec3-x pos))) ", "
     (number->string (round (vec3-y pos))) ", "
     (number->string (round (vec3-z pos))) ")")))
