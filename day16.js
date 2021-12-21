let fs = require("fs");
let input = fs.readFileSync("input/day16.in", "utf-8");

let bits = input
  .split("")
  .map((a) => parseInt(a, 16).toString(2).padStart(4, "0").split(""))
  .reduce((p, c) => [...p, ...c]);

let b2n = (b) => {
  // console.log(b);
  return parseInt(b.join(""), 2);
};

let parse_packet = (p) => {
  // console.log(p);
  //   if(p.length == 0) return [p, {}];
  let version = b2n(p.splice(0, 3));
  let type = b2n(p.splice(0, 3));
  let packet = {
    version,
    type,
    sub: [],
  };
  //   console.log(`Found packet of type ${type} version ${version}`);
  switch (type) {
    case 4:
      {
        let lit = BigInt(0);
        let n = b2n(p.splice(0, 5));
        while (n & 0x10) {
          lit = (lit << BigInt(4)) | BigInt(n & 0xf);
          n = b2n(p.splice(0, 5));
        }
        lit = (lit << BigInt(4)) | BigInt(n & 0xf);
        packet.value = lit;
      }
      break;
    default:
      let len_type = b2n(p.splice(0, 1));

      //   console.log(`len_type: ${len_type}`);
      switch (len_type) {
        case 0:
          {
            let len = b2n(p.splice(0, 15));
            // console.log(`len: ${len}`);
            let new_dat = p.splice(0, len);

            let pak;
            while (new_dat.length > 6) {
              [new_dat, pak] = parse_packet(new_dat);
              packet.sub.push(pak);
            }
          }
          break;
        case 1:
          {
            let num = b2n(p.splice(0, 11));
            let pak;
            // console.log(`num ${num}`);
            for (let i = 0; i < num; i++) {
              [p, pak] = parse_packet(p);
              packet.sub.push(pak);
            }
          }
          break;
      }
      break;
  }
  return [p, packet];
};
console.log(bits);
let [_, packet] = parse_packet(bits);

let sum_packets = (p) => {
  let res = p.version;
  p.sub.forEach((a) => (res += sum_packets(a)));
  return res;
};
console.log(sum_packets(packet));
let interpret = (p) => {
  let op;
  let init = BigInt(0);
  let sub = p.sub.map((a) => interpret(a));
  switch (p.type) {
    case 4:
      console.log(p.value);
      return p.value;
    case 0: //sum
      op = (a, b) => a + b;
      break;
    case 1: //product
      op = (a, b) => a * b;
      init = BigInt(1);
      break;
    case 2: //min
      op = (a, b) => (a < b ? a : b);
      init = sub[0];
      break;
    case 3: //max
      op = (a, b) => (a > b ? a : b);
      init = sub[0];
      break;
    case 5: //greater
      return BigInt(sub[0] > sub[1]);
    case 6: //less
      return BigInt(sub[0] < sub[1]);
    case 7: //equal
      return BigInt(sub[0] == sub[1]);
  }
  return sub.reduce((p, c) => op(p, c), init);
};

console.log(interpret(packet));
