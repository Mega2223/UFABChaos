gen_ln <- function(X, Y) {
    avg_x <- sum(X) / length(X)
    avg_y <- sum(Y) / length(Y)
    scalar <- sum((X - avg_x) * (Y - avg_y)) / sum(((X - avg_x)^2))
    addt <- avg_y - scalar * avg_x
    f <- function(x) {
        (x * scalar + addt)
    }
    return(f)
}

plot_ln <- function(X, Y) {
    f <- gen_ln(X, Y)
    l <- f(X)
    return(matplot(X, matrix(c(Y, l), nrow = 128), type = c("p", "l"), pch = "+"))
}

X <- 1:128
Y <- sin(X) + rnorm(128, sd = 10)
plot_ln(X, Y)
