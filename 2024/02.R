lines <- readLines(con = "stdin", warn = FALSE)

solve1 <- function(v) {
  d <- diff(v)
  (all(d > 0) || all(d < 0)) && min(abs(d)) >= 1 && max(abs(d)) <= 3
}

solve2 <- function(v) {
  if (solve1(v)) return(TRUE)
  for (i in seq_along(v)) {
    if (solve1(v[-i])) return(TRUE)
  }
  FALSE
}

tot <- c(0, 0)
for (l in lines) {
  v <- as.numeric(strsplit(l, " ")[[1]])
  tot[1] <- tot[1] + solve1(v)
  tot[2] <- tot[2] + solve2(v)
}

print(paste0("Part 1: ", tot[1]))
print(paste0("Part 2: ", tot[2]))