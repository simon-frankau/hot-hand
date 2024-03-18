# "Hot hand" probability analysis

There's a paper ["Surprised by the Gambler's and Hot Hand Fallacies? A
Truth in the Law of Small
Numbers"](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=2627354)
going around about counterintuitive probability effects. There are
[blog
articles](https://statmodeling.stat.columbia.edu/2015/07/09/hey-guess-what-there-really-is-a-hot-hand/)
and [Twitter
posts](https://twitter.com/littmath/status/1769408478139785497) about
it, but people have been a bit rubbish about getting at the intuition
behind it. So, that's what I'm trying to do here.

I've not read the paper, because for some reason I can't read
probability papers when economics departments are involved. It always
reads like they can't say what they mean.

My intuition is that you can't push a division through an expectation!
This is not very clear, either, so I shall try to clarify.

## The result

The result is that, if you flip a die $N$ times, the expectation of
the fraction of heads followed by another heads is less than the
fraction of heads followed by a tails. It's not half!

Put another way, if $H$ is a random variable for the number of heads
seen, and $HH$ is a random variable for the number of pairs of heads
seen in the sequence, then $\mathbb{E}(HH/H) \neq 0.5$.

The rest of this doc tries to explain what this is the case.

## The code

For our calculations, we will need to know, for a given number of coin
flips $N$, the probability of, in a sequence of $N+1$ flips, getting
$H$ heads in the first $N$ flips, and $HH$ pairs of consecutive heads
in $N+1$ flips. (Why the $N+1$ discrepancy? This is so that that $H =
HT + HH$.

The included program calculates these values. Or rather it calculates
the number of sequences in each bucket, and you can divide through by
the total number of sequences to get a probability. And then you can
do the analysis...

## The analysis

Using the notation $p(H = i \wedge HH = j)$ for the probability of
getting a sequence with the given values of $H$ and $HH$, the
expectations for $H$ and $HH$ are:

$$
\mathbb{E}(H) = \sum_i i \ P(H = i)
                = \sum_i \sum_j i \ P(H = i \wedge HH = j)
$$

$$
\mathbb{E}(HH) = \sum_j j \ P(HH = j)
                = \sum_i \sum_j j \ P(H = i \wedge HH = j)
$$

So far so normal. We can do the same thing for $\mathbb{E}(HH/H)$:

$$
\mathbb{E}(HH/H) = \sum_i \sum_j j / i \ P(H = i \wedge HH = j)
$$

With $N = 10$, the code gives $\mathbb{E}(H) = 5$ and $\mathbb{E}(HH)
= 2.5$, as you'd expect, and $\mathbb{E}(HH/H) \approx 0.45$.

First I'll try to do some algebra to get some intuition around why
$\mathbb{E}(HH/H)$ isn't half, and then do some calculations.

$$
\begin{array}{rcl}
\mathbb{E}(HH) & = & \sum_i \sum_j j \ P(H = i \wedge HH = j) \\
               & = & \sum_i \sum_j j \ P(HH = j \ | \ H = i) P(H = i) \\
               & = & \sum_i P(H = i) \sum_j j \ P(HH = j \ | \ H = i) \\
			   & = & \sum_i P(H = i) \mathbb{E}(HH \ | \ H = i)
\end{array}
$$

$$
\begin{array}{rcl}
\mathbb{E}(HH/H) & = &  \sum_i \sum_j j / i \ P(H = i \wedge HH = j) \\
                 & = &  \sum_i \sum_j j / i \ P(HH = j \ | \ H = i) P(H = i) \\
				 & = &  \sum_i P(H = i) / i \sum_j j \ P(HH = j \ | \ H = i) \\
                 & = &  \sum_i P(H = i) / i \mathbb{E}(HH \ | \ H = i)
\end{array}
$$

**TODO: Explain all this. Plots, intuitive meaning, etc.**
