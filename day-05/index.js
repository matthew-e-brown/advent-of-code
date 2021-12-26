const { readFileSync } = require('fs');

/**
 * @typedef Line
 * @property {number} xs The x component of the start of the line
 * @property {number} ys The y component of the start of the line
 * @property {number} xe The x component of the end of the line
 * @property {number} ye The y component of the end of the line
 */


/**
 * @param {number} start
 * @param {number} stop
 */
function* range(start, stop, step = 1) {
  const [ check, change ] = (start <= stop)
    ? [ i => i <= stop, n => n + step ]
    : [ i => i >= stop, n => n - step ];

  for (let i = start; check(i); i = change(i)) {
    yield i;
  }
}


/**
 * @param {Line[]} lines
 * @param {boolean} diagonals
 */
function run(lines, diagonals) {

  let w = 0;
  let h = 0;

  for (const { xs, ys, xe, ye } of lines) {
    if (xs > w) w = xs;
    if (xe > w) w = xe;
    if (ys > h) h = ys;
    if (ye > h) h = ye;
  }

  // Compensate for lines being inclusive; (1, 2) -> (4, 2) *includes* x=4
  w += 1;
  h += 1;

  const board = Array(w * h).fill(0);

  for (const { xs, ys, xe, ye } of lines) {
    if (xs == xe || ys == ye) {

      // Check which of the two is the "straight" one
      const [ r, i ] = (xs == xe)
        ? [ range(ys, ye), n => xs * w + n ]  // If it's x
        : [ range(xs, xe), n => n * w + ys ]; // If it's y

      for (const n of r) {
        board[i(n)] += 1;
      }

    } else if (diagonals) {

      const xRange = range(xs, xe);
      const yRange = range(ys, ye);

      // Because the lines are guaranteed to be 45 degrees, we know that our
      // ranges are going to be the same length
      for (let i = 0; i < Math.abs(xs - xe) + 1; i++) {
        const { value: x } = xRange.next();
        const { value: y } = yRange.next();
        board[x * w + y] += 1;
      }

    }
  }

  return board.filter(c => c >= 2).length;
}


/**
 * @param {Line[]} data
 * @returns {number}
 */
function part1(data) {
  return run(data, false);
}

/**
 * @param {Line[]} data
 * @returns {number}
 */
function part2(data) {
  return run(data, true);
}


/** @type {Line[]} */
const input = readFileSync(process.argv[2] || './puzzle-input.txt')
  .toString()
  .split(/\r?\n/g)
  .map(str => {
    const r = /^(\d+), ?(\d+) ?-> ?(\d+), ?(\d+)$/;
    const [ _, xs, ys, xe, ye ] = str.match(r).map(Number);

    if ([ xs, ys, xe, ye ].some(isNaN))
      throw new Error("Points must all be numbers.");
    else
      return { xs, ys, xe, ye };
  });


const result1 = part1(input);
console.log('Part 1: There are', result1, 'points where lines overlap.');

const result2 = part2(input);
console.log('Part 2: There are', result2, 'points where lines overlap.');