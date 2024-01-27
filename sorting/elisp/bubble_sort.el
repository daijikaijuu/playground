;;; bubble_sort.el --- Description -*- lexical-binding: t; -*-
;;
;; Copyright (C) 2024 Domrachev Alexandr
;;
;; Author: Domrachev Alexandr <alexandr.domrachev@gmail.com>
;; Maintainer: Domrachev Alexandr <alexandr.domrachev@gmail.com>
;; Created: января 27, 2024
;; Modified: января 27, 2024
;; Version: 0.0.1
;; Keywords: abbrev bib c calendar comm convenience data docs emulations extensions faces files frames games hardware help hypermedia i18n internal languages lisp local maint mail matching mouse multimedia news outlines processes terminals tex tools unix vc wp
;; Package-Requires: ((emacs "24.3"))
;;
;; This file is not part of GNU Emacs.
;;
;;; Commentary:
;;
;;  Description
;;
;;; Code:
(defun bubble_sort (list)
  "Sort the given list using the Bubble Sort algorithm."
  (let ((n (length list)))
    (dotimes (i n)
      (dotimes (j (- n i 1))
        (when (> (nth j list) (nth (1+ j) list))
          (let ((temp (nth j list)))
            (setcar (nthcdr j list) (nth (1+ j) list))
            (setcar (nthcdr (1+ j) list) temp)))))
    list))

(setq my-list '(4 5 2 0 9 3))
(setq sorted-list (bubble_sort my-list))
(print sorted-list)

(provide 'bubble_sort)
;;; bubble_sort.el ends here
