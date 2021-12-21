let fs = require("fs");

let input = fs
  .readFileSync("input/day15.in", "utf-8")
  .split("\n")
  .map((a) => a.split("").map((a) => +a));

let big_input = [];
for (let ii = 0; ii < 5; ii++) {
  for (let i = 0; i < input.length; i++) {
    let next = [];
    for (let jj = 0; jj < 5; jj++) {
      for (let j = 0; j < input.length; j++) {
        let v = input[i][j] + ii + jj;
        if (v > 9) v -= 9;
        next.push(v);
      }
    }
    big_input.push(next);
  }
}
input = big_input;
let dst = [input.length - 1, input[0].length - 1];

let visited = {};

let q = [
  {
    r: 0,
    c: 0,
    cost: 0,
  },
];

let q_push = (v, c) => {
  if (v.r < 0 || v.r >= input.length) return false;
  if (v.c < 0 || v.c >= input[0].length) return false;
  //   console.log(v, c, visited);
  if (visited[`${v.r},${v.c}`]) return false;
  //   console.log(v);
  visited[`${v.r},${v.c}`] = true;
  let n = {
    ...v,
    cost: c + +input[v.r][v.c],
    g: c + input[v.r][v.c]+dst[0]+dst[1]-v.r-v.c+2
  };
  q.push(n);
  //   q.sort((a, b) => a.cost - b.cost);
  return true;
};
while (q.length) {
  //   console.log(q);
  let next = q.shift();
  let r = next.r;
  let c = next.c;
  //   if (visited[`${r},${c}`]) continue;
  if (dst[0] == r && dst[1] == c) {
    console.log(next.cost);
    break;
  }
  let cost = next.cost; //+ +input[r][c];
  if (
    q_push(
      {
        r: r + 1,
        c,
      },
      cost
    ) |
    q_push(
      {
        r: r - 1,
        c,
      },
      cost
    ) |
    q_push(
      {
        r,
        c: c + 1,
      },
      cost
    ) |
    q_push(
      {
        r,
        c: c - 1,
      },
      cost
    )
  )
    q.sort((a, b) => {
      let ac = a.cost// + (dst[0] - a.r) + (dst[1] - a.c);
      let bc = b.cost// + (dst[0] - b.r) + (dst[1] - b.c);
      return ac - bc;
    });
}
