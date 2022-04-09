use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

fn apply_replacements(molecule: &str, replacements: &[(&str, &str)]) -> HashSet<String> {
    let mut molecules = HashSet::new();
    for (pattern, replacement) in replacements {
        for (index, _) in molecule.match_indices(pattern) {
            let (start, end) = molecule.split_at(index);
            molecules.insert(format!(
                "{}{}{}",
                start,
                replacement,
                end.strip_prefix(pattern).unwrap()
            ));
        }
    }
    molecules
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MoleculeCandidate {
    steps: usize,
    molecule: String,
}

impl Ord for MoleculeCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        // Prefer shorter molecules over longer ones
        // fewer steps over more steps
        // resolve ties by comparing molecules
        other
            .molecule
            .len()
            .cmp(&self.molecule.len())
            .then_with(|| self.steps.cmp(&other.steps))
            .then_with(|| self.molecule.cmp(&other.molecule))
    }
}

impl PartialOrd for MoleculeCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_molecule(
    initial_molecule: &str,
    final_molecule: &str,
    replacements: &[(&str, &str)],
) -> Option<usize> {
    let mut candidates = BinaryHeap::from([MoleculeCandidate {
        steps: 0,
        molecule: initial_molecule.to_string(),
    }]);

    while let Some(MoleculeCandidate { steps, molecule }) = candidates.pop() {
        if molecule == final_molecule {
            return Some(steps);
        }

        for new_molecule in apply_replacements(&molecule, replacements) {
            candidates.push(MoleculeCandidate {
                steps: steps + 1,
                molecule: new_molecule,
            });
        }
    }

    None
}

pub fn main(input: &str) -> anyhow::Result<()> {
    let (replacements, medicine_molecule) = input.split_once("\n\n").unwrap();
    let replacements: Vec<(&str, &str)> = replacements
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .collect();
    let medicine_molecule = medicine_molecule.trim();

    let molecules = apply_replacements(medicine_molecule, &replacements);

    println!(
        "Number of distinct molecules that can be created: {}",
        molecules.len()
    );

    // Start with the medicine molecule instead of "e" based on the assumptions there are fewer
    // paths to explore that way and because it's easier to determine fitness.
    println!(
        "Fewest steps required to create medicine molecule: {:?}",
        find_molecule(
            medicine_molecule,
            "e",
            &replacements
                .iter()
                .map(|(pattern, replacement)| (*replacement, *pattern))
                .collect::<Vec<_>>()
        )
    );

    Ok(())
}
