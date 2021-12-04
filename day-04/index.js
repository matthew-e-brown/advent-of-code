const { readFileSync } = require('fs');

/// God damn this code is kind of ugly. I can't believe I used a class. 


class Board {

  /**
   * @param {string} data
   */
  constructor(data) {
    // Split 2D board into array
    this.numbers = data
      .split('\n')
      .map(line => line
        .split(/\s+/)
        .filter(Boolean)
        .map(s => ({ number: Number(s), marked: false }))
      );

    this.w = this.numbers.length;
    this.h = this.numbers[0].length;

    this.alreadyValidated = false;
  }


  /**
   * @param {number} number
   */
  draw(number) {
    for (let i = 0; i < this.w; i++) {
      for (let j = 0; j < this.h; j++) {
        if (this.numbers[i][j].number == number)
          this.numbers[i][j].marked = true;
      }
    }
  }


  /**
   * @returns {bool}
   */
  isValid() {
    if (this.alreadyValidated) return true;

    // For every (any) row...
    for (let j = 0; j < this.h; j++) {
      // Check if all the columns are true
      if (this.numbers.every(sub => sub[j].marked)) {
        this.alreadyValidated = true;
        return true;
      }
    }

    // For every (any) column...
    for (let i = 0; i < this.w; i++) {
      // Check if all the rows are true
      if (this.numbers[i].every(({ marked }) => marked)) {
        this.alreadyValidated = true;
        return true;
      }
    }

    return false;
  }


  /**
   * @returns {number}
   */
  unmarkedSum() {
    return this.numbers
      .flatMap(sub => sub.map(({ marked, number }) => !marked ? number : 0))
      .reduce((a, c) => a + c);
  }

}


/**
 * @param {number[]} draws
 * @param {Board[]} boards
 */
function part1(draws, boards) {

  let n;

  const firstValid = (() => {
    for (n of draws) {
      for (const board of boards) {
        board.draw(n);
        if (board.isValid()) return board;
      }
    }
  })();

  return firstValid.unmarkedSum() * n;
}


/**
 * @param {number[]} draws
 * @param {Board[]} boards
 */
function part2(draws, boards) {

  /** @type {Board | null} */ 
  let lastComplete = null;
  /** @type {number | null} */
  let completedDraw = null;
  let completed = 0;

  for (const n of draws) {
    for (const board of boards) {
      if (board.alreadyValidated) continue;

      board.draw(n);
      if (board.isValid()) {
        completed += 1;
        lastComplete = board;
        completedDraw = n;
      }

      if (completed == boards.length) return board.unmarkedSum() * n;
    }
  }

  return lastComplete.unmarkedSum() * completedDraw;
}


const rawInput = readFileSync(process.argv[2] || './puzzle-input.txt')
  .toString()
  .split(/(?:\n\n|\r\n\r\n)/);

const draws = rawInput.shift().split(',').map(Number);

// We map in-place for both arguments so we don't have to deal with
// mutability issues from passing an already-used set of boards into part2
// after completing part1

const result1 = part1(draws, rawInput.map(chunk => new Board(chunk)));
console.log(`Part 1: sum of the first valid board is:`, result1);

const result2 = part2(draws, rawInput.map(chunk => new Board(chunk)));
console.log(`Part 2: sum of the last valid board is:`, result2);