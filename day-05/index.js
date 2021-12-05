const { readFileSync } = require('fs');

/**
 * @typedef Line
 * @property {number} xs The x component of the start of the line
 * @property {number} ys The y component of the start of the line
 * @property {number} xe The x component of the end of the line
 * @property {number} ye The y component of the end of the line
 */


/**
 * @param {Line[]} data
 * @returns {number}
 */
function part1(data) {

  let w = 0;
  let h = 0;

  // Only need the straight lines. May as well also find the size of our board
  // while we're at it.
  const filtered = data.filter(({ xs, ys, xe, ye }) => {
    if (xs > w) w = xs;
    if (xe > w) w = xe;
    if (ys > h) h = ys;
    if (ye > h) h = ye;
    return xs == xe || ye == ys;
  });

  console.log(filtered);

  // Account for the line (x,y)s being inclusive
  w += 1;
  h += 1;

  console.log(`Board is ${w}x${h}`);
  const board = Array(w * h).fill(0);

  for (const { xs, ys, xe, ye } of filtered) {

    let start, end, index;

    if (xs == xe) {
      index = n => xs * w + n;
      start = Math.min(ys, ye);
      end = Math.max(ys, ye);
    } else {
      index = n => n * w + ys;
      start = Math.min(xs, xe);
      end = Math.max(xs, xe);
    }

    for (let n = start; n <= end; n++) {
      board[index(n)] += 1;
    }

  }

  return board.filter(c => c >= 2).length;
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
console.log(`Part 1: There are`, result1, `overlapping points.`);