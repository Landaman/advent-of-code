use std::{cmp::max, collections::BTreeMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input file");
    let mut lines_iterator = contents.lines();
    let mut fresh_id_ranges: BTreeMap<u64, u64> = BTreeMap::new();

    for line in lines_iterator.by_ref().take_while(|line| !line.is_empty()) {
        let range = line.split_once("-").expect("Invalid range format");
        let mut start = range.0.parse::<u64>().expect("Invalid number format");
        let mut end = range.1.parse::<u64>().expect("Invalid number format");

        let mut end_of_overlapping_range_before_start: Option<u64> = None;
        let mut start_of_overlapping_range_after_end: Option<u64> = None;

        // If the closest range from before the start ends after our start, we can merge into that
        // one
        if let Some((before_start, before_end)) = fresh_id_ranges.range(..=start).next_back()
            && *before_end >= start
        {
            if *before_end >= end {
                // special case - The new range is fully contained in an existing range, skip it
                continue;
            }

            start = *before_start; // No need to check range, this is guaranteed an extension 
            end_of_overlapping_range_before_start = Some(*before_end);
        }

        // If there's something that starts in our range, we can also merge that. Just make sure
        // we're not going to re-find the start range (or put in a start > end)
        if start != end
            && let Some((after_start, after_end)) = fresh_id_ranges.range(start + 1..=end).next()
        {
            end = max(end, *after_end); // Don't shorten the range if the new range totally contains the old. We want to delete this range if that's the case, so still take it
            start_of_overlapping_range_after_end = Some(*after_start);
        }

        // No range connection
        if start_of_overlapping_range_after_end.is_none()
            && end_of_overlapping_range_before_start.is_none()
        {
            fresh_id_ranges.insert(start, end);
            continue;
        }

        // Now we can assume at least one has changed

        // Extending the start range
        if let Some(end_of_overlapping_range_before_start) = end_of_overlapping_range_before_start
            && start_of_overlapping_range_after_end.is_none()
            && end_of_overlapping_range_before_start < end // Don't shorten the range if the new one is totally consumed by the old
            && let Some(mut_end) = fresh_id_ranges.get_mut(&start)
        {
            *mut_end = end;
        // We already know the start is in our range. Erase and re-create since we can't move the
        // start
        } else if let Some(start_of_overlapping_range_after_end) =
            start_of_overlapping_range_after_end
            && end_of_overlapping_range_before_start.is_none()
        {
            fresh_id_ranges.remove(&start_of_overlapping_range_after_end);
            fresh_id_ranges.insert(start, end); // End is already max
        } else if let Some(start_of_overlapping_range_after_end) =
            start_of_overlapping_range_after_end
            && end_of_overlapping_range_before_start.is_some()
        {
            // Merging two ranges
            fresh_id_ranges.remove(&start_of_overlapping_range_after_end);
            if let Some(mut_end) = fresh_id_ranges.get_mut(&start) {
                *mut_end = end; // End is already guaranteed the furthest
            } else {
                println!(
                    "start: {}, end: {}, start after end: {}",
                    start, end, start_of_overlapping_range_after_end
                );
                panic!("Unreachable state reached when merging ranges (couldn't find start)");
            }
        } else {
            panic!(
                "Unreachable state reached when merging ranges (no overlap but expected overlap)"
            );
        }
    }

    let fresh_count_1 = lines_iterator
        .by_ref()
        .filter(|line| {
            let id = line.parse::<u64>().expect("Invalid ID number");

            if let Some((_, range_end)) = fresh_id_ranges.range(..=id).next_back()
                && *range_end >= id
            {
                true
            } else {
                false
            }
        })
        .count();

    println!("{} IDs are fresh (part one)", fresh_count_1);

    let fresh_count_2 = fresh_id_ranges
        .iter()
        .map(|(start, end)| (end - start) + 1)
        .sum::<u64>();

    println!("{} IDs are fresh (part two)", fresh_count_2);
}
