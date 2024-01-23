;; Define the size of the grid
(setq game-of-life-width 20)
(setq game-of-life-height 20)

;; Initialize the grid with random values
(setq game-of-life-grid
      (make-vector game-of-life-height
                   (make-vector game-of-life-width 0)))

(defun game_of_life\game-of-life-randomize-grid ()
  "Randomly initialize the game grid."
  (dotimes (i game-of-life-height)
    (dotimes (j game-of-life-width)
      (aset (aref game-of-life-grid i) j (if (< (random 2) 1) 1 0)))))

(defun game_of_life\game-of-life-print-grid ()
  "Print the current state of the grid."
  (erase-buffer)
  (dotimes (i game-of-life-height)
    (dotimes (j game-of-life-width)
      (insert (if (= (aref (aref game-of-life-grid i) j) 1) "■" "□")))
    (insert "\n")))

(defun game_of_life\game-of-life-neighbors (i j)
  "Count the number of live neighbors around cell (i, j)."
  (let ((count 0))
    (dotimes (x 3)
      (dotimes (y 3)
        (let ((ni (+ i (- x 1)))
              (nj (+ j (- y 1))))
          (when (and (not (= x 1 y 1))
                     (>= ni 0) (< ni game-of-life-height)
                     (>= nj 0) (< nj game-of-life-width)
                     (= (aref (aref game-of-life-grid ni) nj) 1))
            (setq count (1+ count))))))
    count))

(defun game_of_life\game-of-life-update-grid ()
  "Update the game grid based on the rules of Convay's Game of Life."
  (let ((new-grid (copy-sequence game-of-life-grid)))
    (dotimes (i game-of-life-height)
      (dotimes (j game-of-life-width)
        (let ((neighbors (game_of_life\game-of-life-neighbors i j)))
          (if (= (aref (aref game-of-life-grid i) j) 1)
              (aset (aref new-grid i) j
                    (if (or (< neighbors 2) (> neighbors 3)) 0 1))
            (aset (aref new-grid i) j
                  (if (= neighbors 3) 1 0))))))
    (setq game-of-life-grid new-grid)))

(defun game_of_life\game-of-life-run ()
  "Run the Game of Life simulation."
  (game_of_life\game-of-life-randomize-grid)
  (while t
    (game_of_life\game-of-life-print-grid)
    (sit-for 0.2)
    (game_of_life\game-of-life-update-grid)))


;;; game_of_life.el ends here
