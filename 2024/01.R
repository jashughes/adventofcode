hist_list <- readLines(con = "stdin", warn = FALSE) |>
  strsplit(split = " +") |>
  unlist() |>
  as.numeric() |>
  matrix(byrow = TRUE, ncol = 2) |>
  data.frame() |>
  setNames(c("left", "right"))

distance <- function(hist_list) {
  sum(abs(sort(hist_list$left) - sort(hist_list$right)))
}

get <- function(v, i, d = 0) if (i %in% names(v)) v[[as.character(i)]] else d

similarity <- function(hist_list) {
  counts <- table(hist_list$right)
  sum(vapply(hist_list$left, \(x) x * get(counts, x, 0), numeric(1)))
}

print(paste0("Part 1: ", distance(hist_list)))
print(paste0("Part 2: ", similarity(hist_list)))
