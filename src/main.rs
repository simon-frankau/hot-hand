//
// This simulates flipping a coin NUM_FLIPS times, and counting the
// number of heads and pairs of consecutive heads seen, in order to
// help understand the "hot hand" phenomenon referenced from
// README.md.
//

// Number of flips we'll do the analysis over.
const NUM_FLIPS: usize = 10;

// Print out extra debug info?
const DEBUG: bool = false;

// We count sequences, counting the number of sequences that end in
// heads and tails separately. We use f64 as the numbers may get large
// with large numbers of simulations (the number of sequences grow
// exponentially).
#[derive(Clone, Copy, Debug)]
struct Ways {
    heads: f64,
    tails: f64,
}

fn main() {
    // We want to count the numbers of heads (H) and heads-heads (HH)
    // differently: For the former, we count the number of heads in N
    // flips, for the latter, we count the number of of HH in N+1
    // flips, assuming the next flip is heads (`.heads` element) or
    // tails (`.tails`). We do this so that each H can be matched
    // against a possible HH, so that the number of #HTs should be
    // #Hs - #HHs.

    // num_ways[i][j] contains the number of sequencess that
    // contains i Hs, and j HHs in the extended-by-one sequence.

    // For example, after 3 rolls, the possibilities of having two
    // heads are: HHT, HTH and THH. Assuming the next roll is a tails,
    // there are two ways of making one HH: HHTT and THHT. So,
    // num_ways[2][1].tails = 2. Assuming the next roll is a heads,
    // there are two ways of making one HH: HHTH, HTHH. So,
    // num_ways[2][1].heads = 2.

    let mut num_ways = Vec::new();
    let no_ways = Ways {
        heads: 0.0,
        tails: 0.0,
    };
    for _idx in 0..=NUM_FLIPS {
        // Use f64s, as numbers can get large.
        num_ways.push(vec![no_ways; NUM_FLIPS + 1]);
    }

    // After zero rolls, There is precisely one way to have zero Hs,
    // and zero HHs. Whether you assume the next roll is heads or
    // tails makes no difference.
    num_ways[0][0].heads = 1.0;

    // Iteratively calculate num_ways for increasing numbers of flips.
    for flip_num in 1..=NUM_FLIPS {
        // Iteration and expression order is carefully written to
        // allow in-place update.
        for i in (1..=NUM_FLIPS).rev() {
            // The H implicit in the .head count is not included in
            // the count used by the i index. So, when we extend the
            // sequence, that becomes part of the official
            // count. That's why we always refer to
            // num_ways[i][...].tails, and
            // num_ways[i-1][...].heads. In the latter case, the heads
            // count gets increased.

            for j in (1..=NUM_FLIPS).rev() {
                // In each case, the new sequences are generated by
                // tacking on a head or a tail (respectively) to both
                // the previous iteration's head and tail ending
                // counts.

                // Tacking a heads onto a heads increases the HH count.
                num_ways[i][j].heads = num_ways[i][j].tails + num_ways[i - 1][j - 1].heads;
                // Tacking on a tails does not.
                num_ways[i][j].tails += num_ways[i - 1][j].heads;
            }

            // Edge case.
            num_ways[i][0].heads = num_ways[i][0].tails;
            num_ways[i][0].tails += num_ways[i - 1][0].heads;
        }

        if DEBUG {
            eprintln!("{}: {:?}", flip_num, num_ways);
        }
    }

    // Flatten down to a single number per H/HH count.
    let num_ways = num_ways
        .iter()
        .map(|row| row.iter().map(|w| w.heads + w.tails).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    ////////////////////////////////////////////////////////////////////////
    // Pretty-print as CSV

    // Header line.
    print!("# Hs");
    for j in 0..=NUM_FLIPS {
        print!(",{} HHs", j);
    }
    println!();

    // Body
    for i in 0..=NUM_FLIPS {
        print!("{}", i);
        for j in 0..=NUM_FLIPS {
            print!(",{}", num_ways[i][j]);
        }
        println!();
    }
}
