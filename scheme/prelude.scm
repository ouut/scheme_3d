(require-builtin macroquad)

(provide with-camera-3d
         with-camera-2d)

(define (with-camera-3d cam thunk)
  (push-camera-state)
  (set-camera-3d cam)
  (let ([result (call-with-exception-handler
                 (lambda (err)
                   (pop-camera-state)
                   (raise-error err))
                 (lambda ()
                   (thunk)))])
    (pop-camera-state)
    result))

(define (with-camera-2d cam thunk)
  (push-camera-state)
  (set-camera-2d cam)
  (let ([result (call-with-exception-handler
                 (lambda (err)
                   (pop-camera-state)
                   (raise-error err))
                 (lambda ()
                   (thunk)))])
    (pop-camera-state)
    result))
