const { readFileSync } = require('fs');

const getCommons = (arr) => Array.from({ length: arr[0].length }).map((_, i) => {
  const counts = { 0: 0, 1: 0 };

  for (const s of arr)
    counts[ s[i] ] += 1;

  return (counts[1] >= counts[0] ? '1' : '0');
});


function part1(data) {
  const w = data[0].length;

  const c = getCommons(data);

  let delta = 0;
  for (let i = 0; i < w; i++)
    delta = (delta << 1) | parseInt(c[i], 2);

  const mask = parseInt('1'.repeat(w), 2);
  return delta * (delta ^ mask);
}


function part2(data) {
  const w = data[0].length;

  let validOxygen = [...data];
  let validCarbon = [...data];

  for (let i = 0; i < w; i++) {
    if (validOxygen.length > 1) {
      const c = getCommons(validOxygen);
      validOxygen = validOxygen.filter(bin => bin[i] == c[i]);
    }

    if (validCarbon.length > 1) {
      const c = getCommons(validCarbon);
      validCarbon = validCarbon.filter(bin => bin[i] != c[i]);
    }
  }

  return parseInt(validOxygen, 2) * parseInt(validCarbon, 2);
}


const input = readFileSync(process.argv[2])
  .toString()
  .split('\n')
  .map(s => s.trim());

const result1 = part1(input);
console.log('Part 1:', result1);

const result2 = part2(input);
console.log('Part 2:', result2);