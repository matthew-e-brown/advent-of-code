const { readFileSync } = require('fs');


/** @param {number[]} data */
function part1(data) {

  let largerCount = 0;

  // wow, haven't used a regular for loop in a while
  for (let i = 0; i < data.length - 1; i++) {
    const a = data[i];
    const b = data[i + 1];

    if (b > a) largerCount += 1;
  }

  return largerCount;
}


/** @param {number[]} data */
function part2(data) {

  let largerCount = 0;

  for (let i = 0; i < data.length - 3; i++) {
    const windowA = data[i + 0] + data[i + 1] + data[i + 2];
    const windowB = data[i + 1] + data[i + 2] + data[i + 3];

    if (windowB > windowA) largerCount += 1;
  }

  return largerCount;
}


const input = readFileSync('./puzzle-input.txt')
  .toString()
  .split('\n')
  .map(Number);

const result1 = part1(input);
console.log(`Part 1: ${result1} measurements are larger than their previous.`);

const result2 = part2(input);
console.log(`Part 2: ${result2} windows are larger than their previous.`);