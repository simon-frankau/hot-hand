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
                 & = &  \sum_i P(H = i) / i \ \mathbb{E}(HH \ | \ H = i)
\end{array}
$$

What this means is that both $\mathbb{E}(HH)$ and $\mathbb{E}(HH/H)$
can be thought of as weighted sums over the expectations of $HH$
conditioned on $H$.

For $\mathbb{E}(HH)$, the weights are simply the probabilities of the
values of $H$, and the expectation for the number of $HH$ s comes out
as 1/4 of the number of throws, as you would expect.

For $\mathbb{E}(HH/H)$, the cases where $H$ is large is weighted
less. If $H$ is high, you'd expect $HH$ to dominate over $HT$ (as most
flips are heads!), so by weighing them less you'll decrease the
expectation compared to $\mathbb{E}(HH) / \mathbb{E}(H)$.

To put this much more succinctly, as a mathematician friend did, $H$
and $HH$ are positively correlated, so we expect $\mathbb{E}(HH/H) <
\mathbb{E}(HH) \mathbb{E}(1/H)$. On the other hand, Jensen's
inequality gives $\mathbb{E}(1/H) \geq 1 / \mathbb{E}(H)$, so we can't
be sure exactly sure how $\mathbb{E}(HH/H)$ compares with
$\mathbb{E}(HH/H) / \mathbb{E}(H)$.

### Visualising this

I've added code to calculate $\mathbb{E}(HH \ | \ H = i)$, $P(H = i)$
and $P(H = i) / i$, for the various $i$, so we can visualise the
conditional expectation and the weights being applied:

![Graph of these values. The conditional expectation is weighted
highly to large values of H, while both sets of weights look
normal-ish](./cond_exp.png)

We can see the conditional expectations are highly skewed, so any skew
in the weightings will have a significant effect on the weighted
sum. On this scale, it's hard to compare the weightings, so let's
normalise their heights and take another look:

![Graphs of P(H = i) and P(H = i)/i, normalised and
superimposed](./weights_compared.png)

The graph of $P(H = i)$ is a plot of a straightforward binomial
distribution, totally symmetric. We can see how dividing through by
$i$ skews it - the biggest changes, multiplicatively, will be at the
extremes, but they have very little weight. Even in the centre,
though, we can see how dividing through by $i$ shifts the distribution
leftwards, reducing the expectation for $\mathbb{E}(HH/H)$.

## So what does this mean?

This puts me in mind of change of numeraire: Depending on how you want
to count things, you get different results.

If, for a given number of coin flips, you want to place a bet that
number of heads followed by tails is greater than the number of head
followed by heads, you'll likely come out ahead.

If, for a given number of coin flips, every time a heads comes up, you
bet the next coin will be a tails, you have a probability greater than
a half of coming out ahead.

*However*, in this second case, on average, you'll break even, because
your losses will tend to be on sequences with more heads, so your
losses when you lose tend to be larger than your wins when you win.
