// Number of throws we'll do the analysis over.
const NUM_THROWS: usize = 10;

const DEBUG: bool = false;

#[derive(Clone, Copy, Debug)]
struct Ways {
    heads: f64,
    tails: f64
}

// TODO: Update comments to match the currently-calculated values.

fn main() {
    // num_ways[i][j] contains the number of combinations where there
    // are i Hs in the sequence, and j HHs in the sequence extended by
    // one, given the sequence ends with a heads or tails..
    let mut num_ways = Vec::new();
    let no_ways = Ways { heads: 0.0, tails: 0.0 };
    for _idx in 0..=NUM_THROWS {
	// Use f64s, as numbers can get large.
	num_ways.push(vec!(no_ways; NUM_THROWS + 1));
    }

    num_ways[0][0].tails = 1.0;
    num_ways[0][0].heads = 1.0;
    
    // Iteratively calculate num_ways for increasing numbers of throws.
    for throw_num in 1..=NUM_THROWS {
	// TODO: Optimise for in-place update.
	let mut new = num_ways.clone();

	for i in 1..=NUM_THROWS {
	    for j in 0..=NUM_THROWS  {
		// Number of ways of creating i Hs and j HHs, with a sequence
		// ending in T after n + 1 steps, is just the same as the
		// total number of ways of creating i Hs and j HHs after n
		// steps.
		let new_tails = num_ways[i][j].tails + num_ways[i - 1][j].heads;

		// A sequence ending in H will have one more H than a shorter
		// sequence, and one more HH than a shorter sequence ending in
		// H.
		let new_heads = num_ways[i][j].tails + if j > 0 { num_ways[i-1][j-1].heads } else { 0.0 };

		new[i][j] = Ways { tails: new_tails, heads: new_heads };
	    }
	}

	std::mem::swap(&mut new, &mut num_ways);

	if DEBUG {
	    eprintln!("{}: {:?}", throw_num, num_ways);
	}
    }

    // We report number of ways you can get i Hs in the first
    // NUM_THROWS - 1 throws, and then j HHs in the NUM_THROWS
    // throws. This is so that the number of HTs in NUM_THROWs is i -
    // j.
    //
    // The total number of ways in the previous step is equal to the
    // number of ways in the current step to generate a sequence
    // ending in T.
    // Flatten down to a single number per H/HH count.
    let num_ways = num_ways.iter().map(|row| row.iter().map(|w| w.heads + w.tails).collect::<Vec<_>>()).collect::<Vec<_>>();

    // Pretty-print as CSV

    // Header line.
    print!("# Hs");
    for j in 0..=NUM_THROWS {
	print!(",{} HHs", j);
    }
    println!();

    // Body
    for i in 0..=NUM_THROWS {
	print!("{}", i);
	for j in 0..=NUM_THROWS {
	    print!(",{}", num_ways[i][j]);
	}
	println!();
    }
}
