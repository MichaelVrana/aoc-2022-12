use std::collections::BTreeSet;
use std::collections::VecDeque;

const INPUT: &str = "abcccccccccccccccccccccccccccccccccccccccccccccccccccaaaaacccccccccccccaaaaaaaccccccccccccccccaaaaaaccccccccccccccccccccccccccaaaaacccaaaccccccccccccccccccccccccccccccaaaaa
abcccccccccaaaccccccccccccaaaccccccccccccccccccccccccaaaaaacccccccccccccaaaaaaaaaaaccaaaaccccaaaaaaacccccccccccccccccccccccccccaaaaaacaaaaccccccccccccccccccccccccccccccaaaa
abcccccccccaaaaaacccccccccaaaacccccccccccccccccccccccaaaaaaccccccccccccaaaaaaaaaaaccaaaaacccaaaaaacccccccccaaacccccccccccccccaaaaaaaacaaaaccccccccccccccccacccccccccccccaaaa
abcccccccccaaaaaacccccccccaaaaccccaacccccccccccccccccaaaaaaccccccccccaaaaaaaaaaaaaacaaaaaaccaacaaaccaacccaaaaaaccccccccccccccaaaaaaaacaaaccccccccccaccccaaaccccccccccccaaaaa
abcccccccaaaaaaaccccccccccaaaacccaaaaccccccccccccaaccccaaacccccccaaacaaaaaaaaaccccccaaaaaacccccaaaccaacccaaaaaacccccccccccccccccaaccccccccccccccccaaacccaaaccccccccccccaaaca
abcccccccaaaaaaacccccccaaacccccccaaaacccccccaaccaaaccccccccccccaaaaacaaaaaaaaaccccccaaaaaccccccccaaaaaaaacaaaaaccaacaaccccccccccaaccccccccccccccccaaaaaaaaaccccccccccccccccc
abcccccccccaaaaaaccaaccaaacccccccaaaacccccccaaaaaaaccccccccccccaaaaaaccccaaaaaacccccccaaaccccccccaaaaaaaaaaaaacccaaaaacccccccaaaccccccccccccccccccaaaaaaaackcccccccccccccccc
abcccccccccaaaaaaccaaaaaaaccccccccccccccccccaaaaaacccccccccccccaaaaaaccccaaaaaaccccccccccccccccccccaaaaccaaaaaccccaaaaaccccccaaaaaccccaaaaaccccccccaaaajjkkkkkccccccaacccccc
abcccccccccaaccccccaaaaaaccccccccccccccccccccaaaaaaaaccccccccccaaaaacccaaaacaaccccccccccccccccccccaaaaaccccccccccaaaaaacccccaaaaaaccccaaaaaccciijjjjjjjjjkkkkkkccaaaaaaccccc
abcaaaccccccccccccccaaaaaaaaccccccccccccccccaaaaaaaaacccccccccccaaaacccaaaccaaccccccccccccccccccccaacaaacccccccccaaaacccccccaaaaaacccaaaaaaciiiijjjjjjjjjoopkkkkcaaaaacccccc
abaaaacccccccccccccaaaaaaaaaccccccccccccccccaaaaaaaacccccccccccccccccccaaaaaaaccccaccaccccccccccccacccaacccccccccccaaccccccccaaaaacccaaaaaaiiiiiijjjjjjoooppppkkcaaaaaaacccc
abaaaaaccccccccccccaaaaaaaaccccccccccccccccaaaaaaaccccccccccccccccccccccaaaaaaccccaaaacccccccccccccccccccccccccccccccccccccccaaaaccccaaaaaiiiinnnoooooooooppppkkkaaaaaaacccc
abaaaaacccccccccccaaaaaaaccccccccccccccccccccccaaacccccccccccccccccccaaaaaaaacccccaaaaccccccccccccaaccaacccccaccccacccccccccccccccccccaaaciiinnnnoooooooouupppkkkaaaaaaacccc
abaaaaccccccccccccccccaaaccccccccccccccccccccccaaacccccccaaccccccccccaaaaaaaaacccaaaaaacccccccccccaaaaaaccccaaaaaaaacccccccccccccaaccccccciiinnnntttooouuuuupppiiacaaacccccc
abaaaacccccccccccccccccaaccccccccccccccccccccccccccccccaaaacacccccccccaaaaaaaacccaaaaaacccccccccccaaaaaccccccaaaaaacccccccccccaaaaaacccccciinnnttttuuuuuuuuuuppiiccaaacccccc
abcccccccccccccccccccccccccccccccccccccccccccccccccccccaaaaaacccccccccccaaaaaaaccccaacccccaaccccccaaaaaacccccaaaaaacccccccccccaaaaaccccccciinnntttttuuuuxyuuuppiiicccccccccc
abccccccccccccccccccccccccccccccccccccaaacccccccaaccccccaaaaccccccccccccaaccccccccccccccccaacaaacaaaaaaaacccaaaaaaaaccccccccccaaaaaaaccccchinnnttxxxxuuxyyyuuppiiiiccccccccc
abccccccccccccccccccccccccccccccccccccaaaaaaccccaaacccccaaaaccccccccccccaaccccccccccccccccaaaaaccaaaaaaaaccaaaaaaaaaaccccccccaaaaaaaaccccchhhnnttxxxxxxxyyuvppppiiiccccccccc
abccccccccccccccccccccccccccccccccccaaaaaaaaaaccaaaaaaacacaacccccccccccccccccaaaccccccccaaaaaaccccccaacccccaaaaaaaaaaccccccccaaaaaaaaccccchhhnntttxxxxxxyyvvvppqqiiicccccccc
abccccccccccccccccccccccccccccaacaccaaaaaaaaaaaaaaaaaaacccccccccccccccccccccaaaaaaccccccaaaaaaacccccaacccccaccaaaaacacccccccccacaaaccccccchhhnnnttxxxxxyyyvvvvqqqqiiiccccccc
SbccccccccccccccccccccccccccccaaaaccaaaaaaacaaaaaaaaaaccccccccccccccccccccccaaaaaaccccccccaaaaaaccccccccccccccaaaaccccccccccccccaaacccccchhhmmmtttxxxEzzyyyyvvvqqqqiiccccccc
abcccccccccccccccccccccccccccaaaaaccccaaaaaccaaaaaaaaacccccccccccccccaaaaaccaaaaaaccccccccaaccaacccccccccccccccaacccccccaaaaccccccccccccchhhmmmtttxxxyyyyyyyyvvvqqqjjjcccccc
abcccaaacccccccccccccccccccccaaaaaacccaacaaaaaaaaaaaaacccccccccccccccaaaaacccaaaaaccccccccaacccccccccccccccccccccccccccaaaaacccccccccccchhhmmmttsxxyyyyyyyyvvvvvqqqjjjcccccc
abcccaaaaaacccaacaaccccccccccacaaaacccaacccaaaaaaaaaaaacccccccccccccaaaaaacccaacaaccccccccccccccccccccccccccccccccaacccaaaaaaccccccccccchhhmmmssxxwwwwyyywvvvvvqqqjjjjcccccc
abccaaaaaaacccaaaaaccccccccccccaaccccccccccaaaaaaaccaaccccccccccccccaaaaaacccccccccccccccccccccccccccccccccccaaccaaacccaaaaaacccccccaaachhhmmssswwwwwwyyywvvqqqqqqjjjccccccc
abcaaaaaaacccccaaaaacccccccccccccccccccccccccccaaaccccccccccccccccccaaaaaacccccccccccaaccccaaccccccccccccccccaaacaaacccaaaaaacccccccaaacgggmmsssswwwswwyywwrrqqqjjjjcccccccc
abcaaaaaaaccccaaaaaacccccccccccccccccccccccaaccaaaccccccccccccccccccccaaccccccccccccaaccccaaaacccccccccccccccaaaaaaccccccaacccccccaacaaagggmmmssssssswwwwwwrrqjjjjjddccccccc
abcccaaaaaacccaaaacccccccccccccccccccccccccaaacaaccccccccccccccccccccccccccccccccaaaaacaacaaaaccccccccccccccccaaaaaaaaccccccccccccaaaaaagggmmmmssssssswwwwrrrkjjjjddddcccccc
abcccaaaaaacccccaaccccccccccccccccccccccccccaaaaaccccccccccccccccccccccccccccccccaaaaaaaacaaaacaaccccccaaccaaaaaaaaaaacccccccccccccaaaaaggggmmmmllllsrrwwwrrkkkjdddddaaacccc
abcccaaaccccccccccccaacccccccccccccccccccccaaaaaaccccccccccccccccccccccccccccccccccaaaaacccccccaaaaaaccaaaaaaaaaaaaaacccccccccccccccaaaaaggggmmllllllrrrrrrrkkkdddddaaaccccc
abccccaaaaaaccccccccaacaaaccccccccacccaaccaaaaaaaaccccccccccaaaccccccaacccccccccccaaaaaccccccccaaaaacccaaaaaaaaaaaaccccccccccccccccaaacaacggggggflllllrrrrrrkkddddaaaaaccccc
abccccaaaaacccccccccaaaaacccccccccaaaaaaccaaaaaaaaccccccccccaaacacccaaaaccccccccccaacaaacccccaaaaaaccccaaaaaaaccaaacccccccccccccccccaacccccggggffffllllrrrrkkkdddaaaaaaacccc
abccaaaaaaacccccccaaaaaaccccccccccaaaaaccccccaacccccaaccccaacaaaaaccaaaacccccccaaccccaaccccccaaaaaaaccaaaaaaaaccaaaccccccccccccccccccaaaccccccgffffflllkkkkkkeedaaaaaaaacccc
abccaaaaaaacccccccaaaaaaacccccccccaaaaaacccccaacccccaaacccaaaaaaaaccaaaacccaaaaacccccccccccccccaaaaaacaaaaaaaacccccccccccccccccccccccaaaacaacccccffffllkkkkkeeedccaaaaaacccc
abccccaaaaaaccccccccaaaaaacccccccaaaaaaaacccccaaccccaaaaaaaaaaaaccccccccccaaaaaaaacccccccccccccaaccaaccccaaaccccccaacccccccccccccccccaaaaaaaccccccfffffkkkkeeeecccaacccccccc
abccccaaccaaccccccccaaccaacccccccaaaaaaaacccccaaccaaaaaaaaccaaaaacccccccccaaaaaaaaccccccccccaacaaccccccccaaccccaacaaaccccaacccccccccccaaaaaaccccccaafffeeeeeeecccccccccccccc
abccccaaccccccccccccaaccccccccccccccaacccccaaaaaaaaaaaaaaacaaacaaaaaaacaccaaaaaaacccccccaaacaacccccccccccccccccaaaaaccccaaaacccccccaaaaaaaaccccccaaaaffeeeeeeeccccccccccccca
abacccccccccccccaaacccccccccccccccccaacccccaaaaaaaaaaaaaacccaaccaaaaaaaaaaaaaccaaacccccccaaaaaccccccccccccccccccaaaaaaccaaaacccccccaaaaaaaaaccccccaaacceeeeeccccccccccccccaa
abaaccccccccccaaaaaacccccccccccccccccccccccccaaaacccaaaaaaccccccaaaaaaaaaaaaaaaaaaaccccccaaaaaaacccaaaccccccccaaaaaaaaccaaaaccccccccaaaaaaaaccccccccccccaaacccccccccccaaacaa
abaaccccccccccaaaaaacccccccccccccccccccccccccaaaaacaaaaaaaccccccaaaaaaaaaaaaaaaaaaccccccaaaaaaaaccccaaaaccccccaaaaacaaccccccccccccccccaaaaaaacccccccccccacacccccccccccaaaaaa
abacccccccccccaaaaaaccccccccccccccccccccccccaaaaaacaaaccaaccccccaaaaaaccaaaaaaaaccccccccaaaaaaaaccaaaaaacccccccccaaaccccccccccccccccccaacccccccccccccccccccccccccccccccaaaaa";

type Height = u8;
type Position = (usize, usize);
type Direction = (isize, isize);

type HeightMap = Vec<Vec<Height>>;

struct PathState {
    position: Position,
    steps: isize,
    height: Height,
    prev: Vec<Position>,
}

const START: Height = b'S';
const DESTINATION: Height = b'E';
const LOWEST_HEIGHT: Height = b'a';
const HIGHEST_HEIGHT: Height = b'z';

const UP: Direction = (0, -1);
const DOWN: Direction = (0, 1);
const LEFT: Direction = (-1, 0);
const RIGHT: Direction = (1, 0);

const DIRECTIONS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];

fn find_starting_position(height_map: &HeightMap) -> Position {
    height_map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(
                |(x, height)| {
                    if *height == START {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .unwrap_or_else(|| panic!("Starting point not found"))
}

fn try_direction(
    height_map: &HeightMap,
    path_state: &PathState,
    direction: &Direction,
) -> Option<PathState> {
    let row_count = height_map.len();
    let column_count = height_map
        .first()
        .unwrap_or_else(|| panic!("Heightmap row can't be empty"))
        .len();

    let current_position = path_state.position;

    let next_x = current_position.0 as isize + direction.0;

    if !(0..column_count as isize).contains(&next_x) {
        return None;
    }

    let next_y = current_position.1 as isize + direction.1;

    if !(0..row_count as isize).contains(&next_y) {
        return None;
    }

    let next_height = *height_map
        .get(next_y as usize)
        .map(|row| row.get(next_x as usize))
        .flatten()
        .unwrap();

    if (next_height == DESTINATION && path_state.height != HIGHEST_HEIGHT)
        || (path_state.height + 1 < next_height)
    {
        return None;
    }

    let mut prev = path_state.prev.clone();
    prev.push((next_x as usize, next_y as usize));

    Some(PathState {
        position: (next_x as usize, next_y as usize),
        steps: path_state.steps + direction.0.abs() + direction.1.abs(),
        height: next_height,
        prev,
    })
}

fn main() {
    let height_map: HeightMap = INPUT.lines().map(|line| line.into()).collect();

    let starting_position = find_starting_position(&height_map);

    let mut queue = VecDeque::from([PathState {
        position: starting_position,
        steps: 0,
        height: LOWEST_HEIGHT,
        prev: vec![starting_position],
    }]);

    let mut visited = BTreeSet::from([starting_position]);

    loop {
        let path_state = queue
            .pop_front()
            .unwrap_or_else(|| panic!("No path to destination exists"));

        for next_path_state in DIRECTIONS
            .iter()
            .flat_map(|direction| try_direction(&height_map, &path_state, direction))
        {
            if next_path_state.height == DESTINATION {
                println!("{}", next_path_state.steps);
                return;
            }

            if visited.contains(&next_path_state.position) {
                continue;
            }

            visited.insert(next_path_state.position);
            queue.push_back(next_path_state);
        }
    }
}
