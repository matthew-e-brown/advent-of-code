const { readFileSync } = require('fs');


/**
 * @typedef Instruction
 * @property {'forward' | 'down' | 'up'} dir
 * @property {number} num
 */


/** @param {Instruction[]} data */
function part1(data) {

  let depth = 0;
  let horizontal = 0;

  for (const instruction of data) {

    switch (instruction.dir) {
      case 'forward': horizontal += instruction.num; break;
      case 'up':      depth -= instruction.num; break;
      case 'down':    depth += instruction.num; break;
    }

  }

  return depth * horizontal;

}


/** @param {Instruction[]} data */
function part2(data) {

  let aim = 0;
  let depth = 0;
  let horizontal = 0;

  for (const inst of data) {

    switch (inst.dir) {
      case 'up': aim -= inst.num; break;
      case 'down': aim += inst.num; break;
      case 'forward':
        horizontal += inst.num;
        depth += inst.num * aim;
        break;
    }

  }

  return depth * horizontal;

}


const input = readFileSync(process.argv[2])
  .toString()
  .split('\n')
  .map(line => {
    const [ dir, num ] = line.split(' ');
    return { dir, num: Number(num) };
  });


const result1 = part1(input);
console.log(`Part 1: final product of depth * position: ${result1}`);

const result2 = part2(input);
console.log(`Part 2: final product of depth * position: ${result2}`);