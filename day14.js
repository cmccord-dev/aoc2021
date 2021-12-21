let fs = require("fs");
let input = fs.readFileSync("input/day14.in", "utf-8").split("\n");

let rules = input.slice(2).reduce((p, c) => {
  p[c.split(" -> ")[0]] = c.split(" -> ")[1];
  return p;
}, {});
// console.log(rules);
let str = input[0].split("");

let run_iter = (name) => {
  let str = name.split("");
  let next = [];
  for (let i = 0; i < str.length - 1; i++) {
    next.push(str[i]);
    let r = rules[str[i] + str[i + 1]];
    if (r) next.push(r);
  }
  next.push(str.pop());
  return next.join("");
};

let rule_counts = {};
Object.entries(rules).forEach(([k, v]) => {
  let pair1 = k[0] + v;
  let pair2 = v + k[1];
  rule_counts[k] = [pair1, pair2];
});

let counts = {};
for (let i = 0; i < str.length - 1; i++) {
  let s = str.slice(i, i + 2).join("");
  console.log(s);
  if (!counts[s]) counts[s] = 0;
  counts[s]++;
}
console.log(counts);
for (let i = 0; i < 40; i++) {
  let new_counts = {};
  Object.keys(counts).forEach((c) => {
    let rule = rule_counts[c];
    if (rule) {
      rule.forEach((r) => {
        if (!new_counts[r]) new_counts[r] = 0;
        new_counts[r] += counts[c];
      });
    } else {
      if (!new_counts[c]) new_counts[c] = 0;
      new_counts[c] += counts[c];
    }
  });
  counts = new_counts;
}
// console.log(counts, rule_counts);
{
  let map = {};
  "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("").forEach((a) => (map[a] = 0));
  Object.entries(counts).forEach(([k, v]) => {
    map[k[0]] += v;
    // map[k[1]] += v;
  });

  let min = Infinity;
  let max = 0;
  Object.values(map).forEach((v) => {
    if (v == 0) return;
    if (v < min) min = v;
    if (v > max) max = v;
  });
  console.log(max - min+1);
}

for (let j = 0; j < 10; j++) {
  let next = [];
  for (let i = 0; i < str.length - 1; i++) {
    next.push(str[i]);
    let r = rules[str[i] + str[i + 1]];
    if (r) next.push(r);
  }
  next.push(str.pop());
  str = next;
}
let map = {};
let max = { letter: str[0], count: 0 };
// let min = {letter:str[0], count:Infinity}
while (str.length) {
  let l = str.pop();
  if (map[l] === undefined) map[l] = 0;
  map[l]++;
  if (map[l] > max.count) max = { letter: l, count: map[l] };
  // if(map[l]<min.count) min = {letter:l, count:map[l]};
}
let min = Object.values(map).reduce((p, c) => (p < c ? p : c), Infinity);

console.log(min, max);
console.log(max.count - min);

function* base(input) {
  for (let i = 0; i < input.length; i++) {
    // console.log('Yielding', input.slice(i, i+1));
    yield input.slice(i, i + 1);
  }
}

function* transformer(input) {
  let c = input.next();
  let last;
  while (!c.done) {
    let v = c.value;
    // console.log(last, v);
    if (last) {
      if (rules[last + v]) yield rules[last + v];
    }
    yield v;
    last = v;
    c = input.next();
  }
}

// function counter(input) {
//   let map = {};
//   "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("").forEach((a) => (map[a] = 0));
//   let c = input.next();
//   while (!c.done) {
//     let v = c.value;
//     map[v]++;
//     c = input.next();
//   }
//   let min = Infinity;
//   let max = 0;
//   console.log(map);
//   Object.values(map).forEach((v) => {
//     if (v == 0) return;
//     if (v < min) min = v;
//     if (v > max) max = v;
//   });
//   console.log(max - min);
// }

// // let rounds = [];
// let b = base("NNCB");
// for (let i = 0; i < 40; i++) {
//   b = transformer(b);
// }
// counter(b);
